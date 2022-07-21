use std::fs::{read_to_string, write};

use inkwell::context::Context;
use rustpython_parser::ast;
use rustpython_parser::parser::parse_program;

use crate::codegen::{CodeGen, CodeGenArgs};

pub struct Compiler {}

impl Compiler {
    pub fn new() -> Self {
        Self {}
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
        let mut codegen = CodeGen::new(&context, &module, &builder, CodeGenArgs {});

        let (_doc, statements) = split_doc(&ast);
        for statement in statements {
            if let Err(error) = codegen.emit_stmt(statement) {
                // TODO: Verbose error message
                panic!("{}", error);
            }
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
