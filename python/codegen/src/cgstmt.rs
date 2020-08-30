use std::option::Option::Some;

use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValue;
use inkwell::{FloatPredicate, IntPredicate};

use dsp_compiler_error::{err, LLVMCompileError, LLVMCompileErrorType};
use dsp_compiler_value::value::{Value, ValueHandler, ValueType};
use dsp_python_macros::*;
use dsp_python_parser::ast;

use crate::cgexpr::CGExpr;
use crate::scope::LLVMVariableAccessor;
use crate::CodeGen;

pub trait CGStmt<'a, 'ctx> {
    fn compile_stmt(&mut self, stmt: &ast::Statement) -> Result<(), LLVMCompileError>;
    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        args: &Box<ast::Parameters>,
        body: &ast::Suite,
        returns: &Option<ast::Expression>,
    ) -> Result<(), LLVMCompileError>;
    fn compile_stmt_conditional(
        &mut self,
        cond: Value<'ctx>,
        body: &Vec<ast::Statement>,
        orelse: Option<&Vec<ast::Statement>>,
    ) -> Result<(), LLVMCompileError>;
    fn compile_stmt_while(
        &mut self,
        test: &ast::Expression,
        body: &ast::Suite,
        orelse: &Option<ast::Suite>,
    ) -> Result<(), LLVMCompileError>;
}

impl<'a, 'ctx> CGStmt<'a, 'ctx> for CodeGen<'a, 'ctx> {
    fn compile_stmt(&mut self, stmt: &ast::Statement) -> Result<(), LLVMCompileError> {
        self.set_source_location(stmt.location);
        use dsp_python_parser::ast::StatementType;
        match &stmt.node {
            StatementType::Expression { expression } => {
                self.compile_expr(expression)?;
                Ok(())
            }
            StatementType::FunctionDef {
                is_async,
                name,
                args,
                body,
                decorator_list,
                returns,
            } => {
                if *is_async {
                    panic!(
                        "{:?}\nAsync function is not supported",
                        self.get_source_location()
                    )
                }
                let _decorators = decorator_list;
                self.compile_stmt_function_def(name, args, body, returns)?;
                Ok(())
            }
            StatementType::Assign { targets, value } => {
                if targets.len() > 1 {
                    return err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        "Variable unpacking is not implemented."
                    );
                }
                let target = targets.last().unwrap();
                let name = match &target.node {
                    ast::ExpressionType::Identifier { name } => name,
                    _ => {
                        return err!(
                            self,
                            LLVMCompileErrorType::NotImplemented,
                            "Failed to get assignee."
                        );
                    }
                };
                let value = self.compile_expr(value)?;
                let value_type = value.get_type();

                if let Some(fn_value) = &self.get_fn_value() {
                    let llvm_var = self.locals.load(fn_value, name);
                    let pointer = if let Some(llvm_var) = llvm_var {
                        llvm_var.pointer_value()
                    } else {
                        self.builder
                            .build_alloca(value_type.to_basic_type(self.context), name)
                    };
                    self.builder.build_store(pointer, value.to_basic_value());
                    self.locals.set(fn_value, name, (value_type, pointer));
                } else {
                    let global =
                        self.module
                            .add_global(value_type.to_basic_type(self.context), None, name);
                    global.set_unnamed_addr(true);
                    global.set_initializer(&value.to_basic_value());
                    let pointer = global.as_pointer_value();
                    self.globals.set(name, (value_type, pointer));
                }
                Ok(())
            }
            StatementType::Return { value } => {
                // Outside function
                if self.get_fn_value().is_none() {
                    return err!(
                        self,
                        LLVMCompileErrorType::SyntaxError,
                        "'return' outside function"
                    );
                }
                if let Some(value) = value {
                    let return_value = self.compile_expr(value)?;

                    if return_value.get_type() == ValueType::Void {
                        self.builder.build_return(None);
                    } else {
                        // Type check
                        let fn_type = self
                            .get_fn_value()
                            .unwrap()
                            .get_type()
                            .get_return_type()
                            .unwrap();
                        let value_type = return_value.to_basic_value().get_type();
                        if fn_type != value_type {
                            return err!(
                                self,
                                LLVMCompileErrorType::TypeError,
                                format!("{:?}", fn_type),
                                format!("{:?}", value_type)
                            );
                        }

                        return_value.invoke_handler(
                            ValueHandler::new()
                                .handle_int(&|_, value| self.builder.build_return(Some(&value)))
                                .handle_float(&|_, value| self.builder.build_return(Some(&value))),
                        );
                    }
                } else {
                    self.builder.build_return(None);
                }
                Ok(())
            }
            StatementType::ImportFrom {
                level,
                module,
                names,
            } => {
                let _level = level;
                let target = module.as_ref().expect("Unknown module name");
                let _names = names;
                if target == "core.arduino" {
                    // Builtin
                } else {
                    return err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        "Import is not implemented."
                    );
                }
                Ok(())
            }
            StatementType::If { test, body, orelse } => {
                let cond = self.compile_expr(test)?;

                match orelse {
                    None /*Only if:*/ => {
                        self.compile_stmt_conditional(cond, body, None)?;
                    }
                    Some(statements) => {
                        self.compile_stmt_conditional(cond, body, Some(statements))?;
                    }
                }
                Ok(())
            }
            StatementType::While { test, body, orelse } => {
                self.compile_stmt_while(test, body, orelse)?;
                Ok(())
            }
            StatementType::Pass => Ok(()),
            _ => err!(
                self,
                LLVMCompileErrorType::NotImplemented,
                format!("{:?}", stmt)
            ),
        }
    }

    fn compile_stmt_function_def(
        &mut self,
        name: &String,
        args: &Box<ast::Parameters>,
        body: &ast::Suite,
        returns: &Option<ast::Expression>,
    ) -> Result<(), LLVMCompileError> {
        let mut args_vec: Vec<BasicTypeEnum> = vec![];
        let mut arg_names = vec![];

        for arg in args.args.iter() {
            arg_names.push(&arg.arg);
            if arg.annotation.is_none() {
                return err!(
                    self,
                    LLVMCompileErrorType::SyntaxError,
                    "You must provide type hint for arguments"
                );
            }
            let arg_type;
            match &arg.annotation.as_ref().unwrap().node {
                ast::ExpressionType::Identifier { name } => {
                    arg_type = name;
                }
                _ => {
                    return err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        "Unrecognizable type"
                    );
                }
            }
            match arg_type.as_str() {
                "int" => args_vec.push(self.context.i16_type().into()),
                "float" => args_vec.push(self.context.f32_type().into()),
                _ => {
                    return err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        format!("Unimplemented argument type {}", arg_type)
                    );
                }
            }
        }

        let mut return_type = &String::new();
        if let Some(annotation) = returns {
            match &annotation.node {
                ast::ExpressionType::Identifier { name } => {
                    return_type = name;
                }
                ast::ExpressionType::None => {}
                _ => {
                    return err!(
                        self,
                        LLVMCompileErrorType::NotImplemented,
                        "Unknown return annotation node"
                    );
                }
            }
        }

        let f = match return_type.as_str() {
            "int8" => self.module.add_function(
                name,
                self.context.i8_type().fn_type(&args_vec, false),
                None,
            ),
            "int" => self.module.add_function(
                name,
                self.context.i16_type().fn_type(&args_vec, false),
                None,
            ),
            "float" => self.module.add_function(
                name,
                self.context.f32_type().fn_type(&args_vec, false),
                None,
            ),
            "" | "None" => self.module.add_function(
                name,
                self.context.void_type().fn_type(&args_vec, false),
                None,
            ),

            _ => {
                return err!(
                    self,
                    LLVMCompileErrorType::NotImplemented,
                    format!("Unknown return type {}", return_type)
                );
            }
        };
        let bb = self.context.append_basic_block(f, "");

        self.builder.position_at_end(bb);

        self.set_fn_value(Some(f));
        self.locals.create(self.get_fn_value().unwrap());

        for (i, bv) in f.get_param_iter().enumerate() {
            let v = if bv.is_int_value() {
                bv.into_int_value().set_name(arg_names[i]);
                Value::I16 {
                    value: bv.into_int_value(),
                }
            } else if bv.is_float_value() {
                bv.into_float_value().set_name(arg_names[i]);
                Value::F32 {
                    value: bv.into_float_value(),
                }
            } else {
                return err!(
                    self,
                    LLVMCompileErrorType::NotImplemented,
                    "Unimplemented function argument type"
                );
            };
            let pointer = self
                .builder
                .build_alloca(v.get_type().to_basic_type(self.context), arg_names[i]);
            self.builder.build_store(pointer, bv);
            self.locals.set(
                &self.get_fn_value().unwrap(),
                arg_names[i],
                (v.get_type(), pointer),
            );
        }

        for statement in body.iter() {
            self.compile_stmt(statement)?;
        }

        if f.get_type().get_return_type().is_none() && bb.get_terminator().is_none() {
            self.builder.build_return(None);
        }

        self.set_fn_value(None);
        Ok(())
    }

    fn compile_stmt_conditional(
        &mut self,
        cond: Value<'ctx>,
        body: &Vec<ast::Statement>,
        orelse: Option<&Vec<ast::Statement>>,
    ) -> Result<(), LLVMCompileError> {
        // TODO: Fix
        let parent = self.get_fn_value().unwrap();
        let cond = cond.invoke_handler(cvhandler!(self));

        // If-then block
        let then_bb = self.context.append_basic_block(parent, "if.then");

        // If-else block
        let else_bb = self.context.append_basic_block(parent, "if.else");

        // Unconditional block
        let end_bb = self.context.append_basic_block(parent, "if.end");

        self.builder
            .build_conditional_branch(cond, then_bb, else_bb);

        // Emit the 'then' code.
        self.builder.position_at_end(then_bb);
        for statement in body.iter() {
            self.compile_stmt(statement)?;
        }

        // Then, unconditionally jump to the end block
        self.builder.build_unconditional_branch(end_bb);

        // let _then_bb = self.builder.get_insert_block().unwrap();

        // Move
        self.builder.position_at_end(else_bb);
        // Emit the 'else' code if present.
        if let Some(statements) = orelse {
            for statement in statements.iter() {
                self.compile_stmt(statement)?;
            }
        }

        // Then, unconditionally jump to the end block
        self.builder.build_unconditional_branch(end_bb);

        // let _else_bb = self.builder.get_insert_block().unwrap();

        // Set the cursor at the end
        self.builder.position_at_end(end_bb);
        Ok(())
    }

    fn compile_stmt_while(
        &mut self,
        test: &ast::Expression,
        body: &ast::Suite,
        orelse: &Option<ast::Suite>,
    ) -> Result<(), LLVMCompileError> {
        let parent = self.get_fn_value().unwrap();

        let while_bb = self.context.append_basic_block(parent, "while");
        let loop_bb = self.context.append_basic_block(parent, "while.body");
        let else_bb = self.context.append_basic_block(parent, "while.else");
        let end_bb = self.context.append_basic_block(parent, "while.end");

        // Switch to the loop block.
        self.builder.build_unconditional_branch(while_bb);
        self.builder.position_at_end(while_bb);

        // Declare the variable in condition.
        let start = self.compile_expr(test)?;
        let cond = start.invoke_handler(cvhandler!(self));

        // At first, Check whether or not the condition in the header of the loop is true.
        self.builder
            .build_conditional_branch(cond, loop_bb, else_bb);

        // Emit the loop body.
        self.builder.position_at_end(loop_bb);
        for statement in body.iter() {
            self.compile_stmt(statement)?;
        }

        // Emit the conditional branch at the end of the loop body.
        // self.builder
        //     .build_conditional_branch(cond, while_bb, else_bb);
        // It is not needed to check the condition.
        // Return to the while header block and check it in there.
        self.builder.build_unconditional_branch(while_bb);

        // Emit the 'else' code if present.
        self.builder.position_at_end(else_bb);
        if let Some(statements) = orelse {
            for statement in statements.iter() {
                self.compile_stmt(statement)?;
            }
        }

        // Then, unconditionally jump to the end block.
        self.builder.build_unconditional_branch(end_bb);

        // Set the cursor at the end
        self.builder.position_at_end(end_bb);

        Ok(())
    }
}
