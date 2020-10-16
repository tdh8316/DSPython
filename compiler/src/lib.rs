use std::fs::{read_dir, read_to_string};
use std::io::Write;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::support::LLVMString;
use inkwell::targets::{TargetData, TargetTriple};

use dsp_compiler_error::{LLVMCompileError, LLVMCompileErrorType};
use dsp_python_codegen::{CodeGen, get_doc};
use dsp_python_parser::{ast, CompileError};
use dsp_python_parser::parser::parse_program;

pub use crate::flags::*;
use crate::llvm_prototypes::generate_prototypes;

pub mod flags;
mod llvm_prototypes;

type CompileResult<T> = Result<T, LLVMCompileError>;

pub struct Compiler<'a, 'ctx> {
    pub source_path: String,
    pub compiler_flags: CompilerFlags<'a>,

    cg: CodeGen<'a, 'ctx>,
    pass_manager: PassManager<Module<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        source_path: String,
        compiler_flags: CompilerFlags<'a>,
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        pass_manager: PassManager<Module<'ctx>>,
    ) -> Self {
        Compiler {
            source_path,
            compiler_flags,
            cg: CodeGen::new(context, builder, module),
            pass_manager,
        }
    }

    pub fn prepare_module(&mut self) -> CompileResult<()> {
        let libs = read_dir("./arduino/").expect("Failed to load DSPython Arduino core libraries").map(
            |res| res.map(
                |entry| entry.path()
            )
        ).collect::<Result<Vec<_>, std::io::Error>>().unwrap();

        for lib in libs.iter() {
            let lib = lib.to_str().unwrap();
            let to_compile_error =
                |parse_error| CompileError::from_parse_error(parse_error, lib.to_string());

            let source = read_to_string(&lib)
                .expect(&format!("dspython: can't open file '{}'", lib));
            let parsed_ast = parse_program(&source).map_err(to_compile_error);
            if let Err(e) = parsed_ast {
                panic!("ParseError: {}", e);
            }
            let source_ast = parsed_ast.unwrap();
            if let Err(mut e) = self.compile(source_ast) {
                // Enrich error
                e.file = Some(lib.to_string());

                return Err(e);
            }

        }

        Ok(())
    }

    pub fn compile(&mut self, program: ast::Program) -> CompileResult<()> {
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

    pub fn run_pm(&self) {
        self.pass_manager.run_on(&self.cg.module);
    }

    pub fn emit(&self) -> LLVMString {
        self.cg.module.print_to_string()
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

    // Create Compiler instance
    let mut compiler = Compiler::new(
        source_path.clone(),
        flags,
        &context,
        &builder,
        &module,
        pass_manager,
    );

    let source = read_to_string(&source_path)
        .expect(&format!("dspython: can't open file '{}'", source_path));
    let parsed_ast = parse_program(&source).map_err(to_compile_error);
    if let Err(e) = parsed_ast {
        panic!("ParseError: {}", e);
    }

    // Including all default functions is too expensive.
    // TODO: Compile only used functions, not all
    generate_prototypes(compiler.cg.module, compiler.cg.context);
    let source_ast = parsed_ast.unwrap();
    compiler.prepare_module();
    if let Err(mut e) = compiler.compile(source_ast) {
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
