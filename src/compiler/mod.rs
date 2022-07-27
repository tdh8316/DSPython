use std::fs::{read_to_string, write};
use std::process::exit;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::OptimizationLevel;
use rustpython_parser::ast;
use rustpython_parser::parser::parse_program;

use crate::codegen::{CodeGen, CodeGenArgs};

pub struct Compiler {
    optimization_level: u32,
    size_level: u32,
}

impl Compiler {
    pub fn new(optimization_level: u32, size_level: u32) -> Self {
        Self {
            optimization_level,
            size_level,
        }
    }

    /// Compile the given file and return the generated file
    pub fn compile_file(&self, source_path: &str) -> String {
        let source = read_to_string(source_path)
            .expect(format!("Failed to read file {}", source_path).as_str());
        let ast = parse_program(source.as_str())
            .expect(format!("Failed to parse file '{}'", source_path).as_str());

        let context = Context::create();
        let builder = context.create_builder();

        // Create the main module
        let module = context.create_module(source_path);
        // Create the pass manager
        let pm_builder = PassManagerBuilder::create();
        pm_builder.set_optimization_level(match self.optimization_level {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Less,
            2 => OptimizationLevel::Default,
            3 => OptimizationLevel::Aggressive,
            _ => {
                panic!("Invalid optimization level: {}", self.optimization_level);
            }
        });
        pm_builder.set_size_level(self.size_level);
        let pm: PassManager<Module> = PassManager::create(());
        pm_builder.populate_module_pass_manager(&pm);

        let mut codegen = CodeGen::new(&context, &module, &builder, CodeGenArgs {});

        let (_doc, statements) = split_doc(&ast);
        for statement in statements {
            if let Err(error) = codegen.emit_stmt(statement) {
                // TODO: Verbose error message
                eprintln!("{}", error);
                eprintln!(
                    "File: \"{}\", {}",
                    source_path,
                    codegen.get_source_location()
                );
                exit(101);
            }
        }

        // if let Err(error) = module.verify() {
        //     eprintln!("Module verify failed: {}", error);
        //     exit(101);
        // }

        if self.optimization_level > 0 {
            pm.run_on(&module);
        }

        let output_path = format!("{}.ll", source_path);
        write(output_path.as_str(), codegen.emit()).expect("Failed to write LLVM IR");

        output_path
    }
}

/// Split docstring and statements from the AST
pub fn split_doc(body: &[ast::Stmt]) -> (Option<String>, &[ast::Stmt]) {
    if let Some((val, body_rest)) = body.split_first() {
        if let ast::StmtKind::Expr { value } = &val.node {
            if let Some(doc) = try_get_constant_string(std::slice::from_ref(value)) {
                return (Some(doc), body_rest);
            }
        }
    }
    (None, body)
}

fn try_get_constant_string(values: &[ast::Expr]) -> Option<String> {
    fn get_constant_string_inner(out_string: &mut String, value: &ast::Expr) -> bool {
        match &value.node {
            ast::ExprKind::Constant {
                value: ast::Constant::Str(s),
                ..
            } => {
                out_string.push_str(s);
                true
            }
            ast::ExprKind::JoinedStr { values } => values
                .iter()
                .all(|value| get_constant_string_inner(out_string, value)),
            _ => false,
        }
    }
    let mut out_string = String::new();
    if values
        .iter()
        .all(|v| get_constant_string_inner(&mut out_string, v))
    {
        Some(out_string)
    } else {
        None
    }
}
