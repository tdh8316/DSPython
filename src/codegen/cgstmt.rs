use inkwell::types::BasicMetadataTypeEnum;
use rustpython_parser::ast;

use crate::codegen::CodeGen;
use crate::codegen::errors::CodeGenError;
use crate::codegen::symbol_table::{Symbol, SymbolScope, SymbolValueTrait};
use crate::codegen::value::ValueType;
use crate::compiler::split_doc;

impl<'a, 'ctx> CodeGen<'a, 'ctx> {
    pub fn emit_stmt(&mut self, stmt: &ast::Stmt) -> Result<(), CodeGenError> {
        self.set_source_location(stmt.location);
        use ast::StmtKind::*;
        match &stmt.node {
            Expr { value } => {
                self.emit_expr(value)?;
                Ok(())
            }

            // DSPython requires type annotation for variable declaration.
            Assign { .. } => Err(CodeGenError::CompileError(
                "Must have type hint".to_string(),
            )),
            AnnAssign {
                target,
                annotation,
                value,
                ..
            } => self.emit_ann_assign(target, annotation, value),
            FunctionDef {
                name,
                args,
                body,
                decorator_list,
                returns,
                type_comment,
            } => self.emit_function_def(name, args, body, decorator_list, returns, type_comment),
            Return { value } => self.emit_return(value),

            _ => Err(CodeGenError::Unimplemented(format!("stmt: {:#?}", stmt))),
        }
    }

    fn emit_function_def(
        &mut self,
        name: &str,
        args: &ast::Arguments,
        body: &[ast::Stmt],
        decorator_list: &[ast::Expr],
        returns: &Option<Box<ast::Expr>>,
        type_comment: &Option<String>,
    ) -> Result<(), CodeGenError> {
        // RustPython Parser does not support function type comment yet.
        // if type_comment.is_none() {
        //     return Err(CodeGenError::CompileError("Must have type hint".to_string()));
        // }
        // As a workaround, we use the docstring to determine the return type of the function.
        // If the docstring is empty, we assume the function returns None.
        // """
        // @return <type>
        // """
        let (doc, statements) = split_doc(body);
        let return_type_string = if let Some(doc) = doc {
            doc.split("\n")
                .collect::<Vec<&str>>()
                .iter()
                .find_map(|line: &&str| {
                    if line.trim().starts_with("@return") {
                        // Wow, Rust is so beautiful.
                        // Incredibly easy to read!
                        let type_str = line
                            .split("@return ")
                            .nth(1)
                            .unwrap()
                            .split(" ")
                            .nth(0)
                            .unwrap();
                        Some(type_str.to_string())
                    } else {
                        None
                    }
                })
                .unwrap_or("None".to_string())
        } else {
            "None".to_string()
        };

        // Create a new symbol table of the function namespace.
        let mut symbol_table = self.symbol_tables.push_namespace(name.to_string());

        // Types of arguments are determined by the type comment.
        let mut param_types: Vec<BasicMetadataTypeEnum<'ctx>> = Vec::new();
        let args_iter = std::iter::empty()
            .chain(&args.posonlyargs)
            .chain(&args.args)
            .chain(&args.kwonlyargs)
            .chain(args.vararg.as_deref())
            .chain(args.kwarg.as_deref());
        for arg in args_iter {
            if arg.node.annotation.is_none() {
                return Err(CodeGenError::CompileError(
                    "Must have type hint".to_string(),
                ));
            }
            println!("type annotation {:?}", arg.node.annotation);
            // TODO: function arguments
        }

        // Create function
        let function_type = match return_type_string.as_str() {
            "None" => self.context.void_type().fn_type(&param_types, false),
            "int" => self.context.i32_type().fn_type(&param_types, false),
            "float" => self.context.f32_type().fn_type(&param_types, false),
            "str" => {
                return Err(CodeGenError::CompileError(
                    "str is not supported yet".to_string(),
                ));
            }
            "bool" => self.context.bool_type().fn_type(&param_types, false),
            _ => {
                return Err(CodeGenError::CompileError(
                    "Unsupported return type".to_string(),
                ));
            }
        };
        let f = self.module.add_function(name, function_type, None);

        // Create entry block and set it as the current block.
        let bb = self.context.append_basic_block(f, "entry");
        self.builder.position_at_end(bb);

        for statement in statements {
            self.emit_stmt(statement)?;
        }

        // Pop the current namespace.
        self.symbol_tables.pop_namespace();

        Ok(())
    }

    fn emit_return(&mut self, value: &Option<Box<ast::Expr>>) -> Result<(), CodeGenError> {
        if let Some(value) = value {
            // Evaluate the expression if the return value is specified.
            let value = self.emit_expr(value)?;
            match &value.get_type() {
                ValueType::None => self.builder.build_return(None),
                _ => self.builder.build_return(Some(&value.to_basic_value())),
            };
        } else {
            // Return None if the return value is not specified.
            self.builder.build_return(None);
        }
        Ok(())
    }

    fn emit_ann_assign(
        &mut self,
        target: &Box<ast::Expr>,
        annotation: &Box<ast::Expr>,
        value: &Option<Box<ast::Expr>>,
    ) -> Result<(), CodeGenError> {
        let value_type = self.get_value_type_from_annotation(annotation)?;

        let symbol_str = self.get_symbol_str_from_expr(target)?;
        let maybe_symbol = self.symbol_tables.context().get_symbol(symbol_str.as_str());
        let pointer = if let Some(symbol) = maybe_symbol {
            // If the symbol already exists, load it from the symbol table and assign the value.
            symbol.value.get_pointer()
        } else {
            // If the symbol does not exist, declare it and assign the value.
            self.builder
                .build_alloca(value_type.to_basic_type(self.context), symbol_str.as_str())
        };

        // Evaluate the value if it is specified.
        let value = if let Some(value) = value {
            self.emit_expr(value)?
        } else {
            return Err(CodeGenError::CompileError(format!(
                "Cannot assign None to {}",
                symbol_str
            )));
        };

        // Type checker
        if value.get_type() != value_type {
            return Err(CodeGenError::TypeError(format!("{:?}", value_type), format!("{:?}", value.get_type())));
        }

        self.builder.build_store(pointer, value.to_basic_value());

        // Add the symbol to the symbol table.
        // Update the symbol if the symbol already exists.
        self.symbol_tables.context().add_symbol(Symbol::new(
            symbol_str,
            (value_type, pointer),
            SymbolScope::Local,
        ));

        Ok(())
    }

    fn get_symbol_str_from_expr(&mut self, expr: &ast::Expr) -> Result<String, CodeGenError> {
        use ast::ExprKind::*;
        match &expr.node {
            Name { id, .. } => Ok(id.to_string()),
            _ => Err(CodeGenError::CompileError(format!(
                "Cannot get symbol name from {:?}",
                expr
            ))),
        }
    }

    fn get_value_type_from_annotation(
        &mut self,
        annotation: &ast::Expr,
    ) -> Result<ValueType, CodeGenError> {
        match &annotation.node {
            ast::ExprKind::Name { id, .. } => match id.as_str() {
                "int" => Ok(ValueType::I32),
                "float" => Ok(ValueType::F32),
                "str" => Ok(ValueType::Str),
                "bool" => Ok(ValueType::Bool),
                _ => Err(CodeGenError::CompileError(format!(
                    "Unsupported type {}",
                    id
                ))),
            },
            _ => Err(CodeGenError::CompileError(format!(
                "Cannot determine type from {:?}",
                annotation
            ))),
        }
    }
}
