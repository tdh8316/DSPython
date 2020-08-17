use std::fs::read_to_string;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::OptimizationLevel;
use inkwell::passes::{PassManager, PassManagerBuilder};
use inkwell::support::LLVMString;
use inkwell::targets::{TargetData, TargetTriple};

use dsp_compiler_error::LLVMCompileError;
use dsp_python_codegen::cgexpr::CGExpr;
use dsp_python_codegen::cgstmt::CGStmt;
use dsp_python_codegen::CodeGen;
use dsp_python_parser::{ast, CompileError};
use dsp_python_parser::parser::parse_program;

type CompileResult<T> = Result<T, LLVMCompileError>;

pub struct CompilerFlags {
    pub optimization_level: u8,
}

impl CompilerFlags {
    pub fn new(optimization_level: u8) -> Self {
        CompilerFlags { optimization_level }
    }
}

pub struct Compiler<'a, 'ctx> {
    pub program: ast::Program,
    pub source_path: String,
    pub compiler_flags: CompilerFlags,

    cg: CodeGen<'a, 'ctx>,
    pass_manager: PassManager<Module<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        program: ast::Program,
        source_path: String,
        compiler_flags: CompilerFlags,
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
        pass_manager: PassManager<Module<'ctx>>,
    ) -> Self {
        Compiler {
            program,
            source_path,
            compiler_flags,
            cg: CodeGen::new(context, builder, module),
            pass_manager,
        }
    }

    pub fn compile(&mut self) -> CompileResult<()> {
        for statement in self.program.statements.iter() {
            if let ast::StatementType::Expression { ref expression } = statement.node {
                self.cg.compile_expr(&expression)?;
            } else {
                self.cg.compile_stmt(&statement)?;
            }
        }
        Ok(())
    }

    pub fn emit_llvm(&self) -> CompileResult<LLVMString> {
        self.pass_manager.run_on(&self.cg.module);
        Ok(self.cg.module.print_to_string())
    }
}

pub fn compile(source_path: String, flags: CompilerFlags) -> CompileResult<LLVMString> {
    let to_compile_error =
        |parse_error| CompileError::from_parse_error(parse_error, source_path.clone());
    let source = read_to_string(&source_path).unwrap();
    let ast = parse_program(&source)
        .map_err(to_compile_error)
        .expect(&format!(
            "Failed to parse {} because of error above.",
            source_path
        ));

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
    pm_builder.set_optimization_level(OptimizationLevel::Aggressive);
    pm_builder.populate_module_pass_manager(&pass_manager);

    let mut compiler = Compiler::new(
        ast,
        source_path,
        flags,
        &context,
        &builder,
        &module,
        pass_manager,
    );

    compiler.compile();

    compiler.emit_llvm()
}
