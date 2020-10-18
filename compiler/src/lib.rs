use std::fs::{read_dir, read_to_string};
use std::io::Write;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::support::LLVMString;
use inkwell::targets::{TargetData, TargetTriple};
use inkwell::OptimizationLevel;

use dsp_compiler_error::{LLVMCompileError, LLVMCompileErrorType};
use dsp_python_codegen::{get_doc, CodeGen};
use dsp_python_parser::parser::parse_program;
use dsp_python_parser::{ast, CompileError};

pub use crate::flags::*;
use crate::llvm_prototypes::generate_prototypes;

pub mod flags;
mod llvm_prototypes;

type CompileResult<T> = Result<T, LLVMCompileError>;

pub struct Compiler<'a, 'ctx> {
    pub source_path: String,
    pub compiler_flags: CompilerFlags<'a>,

    cg: CodeGen<'a, 'ctx>,
    checker: Checker,
    pass_manager: PassManager<Module<'ctx>>,
    program: ast::Program,
}

pub struct Checker {
    function_has_declared: Vec<String>,
}

impl Checker {
    pub fn new() -> Self {
        Checker {
            function_has_declared: Vec::new(),
        }
    }
    pub fn chk_stmt(&mut self, stmt: &ast::Statement) -> Result<(), LLVMCompileError> {
        match &stmt.node {
            ast::StatementType::Expression { expression } => self.chk_expr(expression)?,
            ast::StatementType::FunctionDef {
                is_async: _is_async,
                name: _name,
                args: _args,
                body,
                ..
            } => {
                for s in body.iter() {
                    self.chk_stmt(s)?;
                }
            }
            ast::StatementType::If { test, body, orelse } => {
                self.chk_expr(test)?;
                for s in body.iter() {
                    self.chk_stmt(s)?;
                }
                if let Some(orelse) = orelse {
                    for s in orelse.iter() {
                        self.chk_stmt(s)?;
                    }
                }
            }
            ast::StatementType::While { test, body, orelse } => {
                self.chk_expr(test)?;
                for s in body.iter() {
                    self.chk_stmt(s)?;
                }
                if let Some(orelse) = orelse {
                    for s in orelse.iter() {
                        self.chk_stmt(s)?;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }

    pub fn chk_expr(&mut self, expr: &ast::Expression) -> Result<(), LLVMCompileError> {
        use dsp_python_parser::ast::ExpressionType;
        match &expr.node {
            ExpressionType::Call { function, .. } => {
                self.function_has_declared.push(match &function.node {
                    ast::ExpressionType::Identifier { name } => name.to_string(),
                    _ => panic!("Unrecognized callee name {:?}", expr),
                });
            }
            _ => {}
        }
        Ok(())
    }
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        source_path: String,
        compiler_flags: CompilerFlags<'a>,
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        pass_manager: PassManager<Module<'ctx>>,
        program: ast::Program,
    ) -> Self {
        Compiler {
            source_path,
            compiler_flags,
            cg: CodeGen::new(context, builder, module),
            checker: Checker::new(),
            pass_manager,
            program,
        }
    }

    pub fn compile(&mut self) -> CompileResult<()> {
        let (statements, _doc_string) = get_doc(&self.program.statements);

        // Add `setup` and `loop` function now, preventing from compiling the same name
        self.cg
            .compile_context
            .function_has_declared
            .append(&mut vec!["setup".to_string(), "loop".to_string()]);

        for statement in statements.iter() {
            if let ast::StatementType::Expression { ref expression } = statement.node {
                self.cg.compile_expr(&expression)?;
            } else {
                self.cg.compile_stmt(&statement)?;
            }
        }
        Ok(())
    }

    pub fn prepare_module(&mut self) -> CompileResult<()> {
        self.get_need_to_define()?;
        let libs = read_dir("./arduino/")
            .expect("Failed to load DSPython Arduino core libraries")
            .map(|res| res.map(|entry| entry.path()))
            .collect::<Result<Vec<_>, std::io::Error>>()
            .unwrap();

        for lib in libs.iter() {
            let lib = lib.to_str().unwrap();
            // Skip `__init__.py` which is for linter
            if lib.contains("__init__.py") {
                continue;
            }
            let to_compile_error =
                |parse_error| CompileError::from_parse_error(parse_error, lib.to_string());

            let source =
                read_to_string(&lib).expect(&format!("dspython: can't open file '{}'", lib));
            let parsed_ast = parse_program(&source).map_err(to_compile_error);
            if let Err(e) = parsed_ast {
                panic!("ParseError: {}", e);
            }
            let source_ast = parsed_ast.unwrap();
            if let Err(mut e) = self.compile_module(source_ast) {
                // Enrich error
                e.file = Some(lib.to_string());

                return Err(e);
            }
        }

        Ok(())
    }

    pub fn run_pm(&self) {
        self.pass_manager.run_on(&self.cg.module);
    }

    pub fn emit(&self) -> LLVMString {
        self.cg.module.print_to_string()
    }

    // TODO: Remove global variables that never used
    pub fn get_need_to_define(&mut self) -> Result<(), LLVMCompileError> {
        for statement in self.program.statements.iter() {
            if let ast::StatementType::Expression { ref expression } = statement.node {
                self.checker.chk_expr(&expression)?;
            } else {
                self.checker.chk_stmt(&statement)?;
            }
        }
        self.cg.compile_context.function_has_declared = self.checker.function_has_declared.clone();

        Ok(())
    }

    pub fn compile_module(&mut self, program: ast::Program) -> CompileResult<()> {
        let (statements, _doc_string) = get_doc(&program.statements);

        for statement in statements.iter() {
            if let ast::StatementType::Expression { ref expression } = statement.node {
                self.cg.compile_expr(&expression)?;
            } else {
                self.cg.compile_stmt(&statement)?;
            }
        }
        Ok(())
    }
}

pub fn get_assembly(source_path: String, flags: CompilerFlags) -> CompileResult<LLVMString> {
    // print!("Compiling {}...", &source_path);
    std::io::stdout().flush().unwrap_or_default();

    let to_compile_error =
        |parse_error| CompileError::from_parse_error(parse_error, source_path.clone());

    let context = Context::create();
    let module = context.create_module(&source_path);
    // Create target data structure for Arduino
    let target_data = TargetData::create("e-P1-p:16:8-i8:8-i16:8-i32:8-i64:8-f32:8-f64:8-n8-a:8");
    module.set_data_layout(&target_data.get_data_layout());
    // LLVM triple
    module.set_triple(&TargetTriple::create("avr"));
    // Create a root builder context
    let builder = context.create_builder();

    // Initialize pass manager
    let pass_manager: PassManager<Module> = PassManager::create(());
    let pm_builder = PassManagerBuilder::create();
    pm_builder.set_optimization_level(match flags.optimization_level {
        0 => OptimizationLevel::None,
        1 => OptimizationLevel::Less,
        2 => OptimizationLevel::Default,
        3 => OptimizationLevel::Aggressive,
        _ => {
            return Err(LLVMCompileError::new(
                None,
                LLVMCompileErrorType::NotImplemented(
                    "Optimization level must be a integer of 0~3".to_string(),
                ),
            ));
        }
    });
    pm_builder.populate_module_pass_manager(&pass_manager);

    let source = read_to_string(&source_path)
        .expect(&format!("dspython: can't open file '{}'", source_path));
    let parsed_ast = parse_program(&source).map_err(to_compile_error);
    if let Err(e) = parsed_ast {
        panic!("ParseError: {}", e);
    }
    let source_ast = parsed_ast.unwrap();

    // Create Compiler instance
    let mut compiler = Compiler::new(
        source_path.clone(),
        flags,
        &context,
        &builder,
        &module,
        pass_manager,
        source_ast,
    );

    // Including all default functions is too expensive.
    generate_prototypes(compiler.cg.module, compiler.cg.context);
    compiler.prepare_module()?;
    if let Err(mut e) = compiler.compile() {
        // Enrich error
        e.file = Some(compiler.source_path);

        return Err(e);
    }

    compiler.run_pm();
    // println!("[Done]");
    {
        Ok(compiler.emit())
    }
}
