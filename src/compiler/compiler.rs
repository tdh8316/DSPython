use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};
use inkwell::{FloatPredicate, IntPredicate};
use rustpython_parser::ast;

use crate::compiler::prototypes::generate_prototypes;
use crate::irgen::expr::CGExpr;
use crate::irgen::stmt::CGStmt;
use crate::value::{Value, ValueHandler, ValueType};

#[derive(Debug, Clone, Copy)]
pub(crate) struct CompileContext {
    pub in_loop: bool,
    pub func: bool,
    pub ret: bool,
}

#[derive(Debug)]
pub struct Compiler<'a, 'ctx> {
    pub context: &'ctx Context,
    pub builder: &'a Builder<'ctx>,
    pub module: &'a Module<'ctx>,

    source_path: String,
    pub(crate) current_source_location: ast::Location,
    pub(crate) ctx: CompileContext,
    pub(crate) variables: HashMap<String, (ValueType, PointerValue<'ctx>)>,
    pub(crate) fn_value_opt: Option<FunctionValue<'ctx>>,
}

impl<'a, 'ctx> Compiler<'a, 'ctx> {
    pub fn new(
        source_path: String,
        context: &'ctx Context,
        builder: &'a Builder<'ctx>,
        module: &'a Module<'ctx>,
    ) -> Self {
        Compiler {
            source_path,
            current_source_location: ast::Location::default(),
            ctx: CompileContext {
                in_loop: false,
                func: false,
                ret: false,
            },
            context,
            builder,
            module,
            variables: HashMap::new(),
            fn_value_opt: None,
        }
    }

    pub fn set_source_location(&mut self, location: ast::Location) {
        self.current_source_location = location;
    }

    #[inline]
    pub fn get_function(&self, name: &str) -> Option<FunctionValue<'ctx>> {
        self.module.get_function(name)
    }

    #[inline]
    pub fn fn_value(&self) -> FunctionValue<'ctx> {
        self.fn_value_opt.unwrap()
    }

    pub fn compile_op(
        &mut self,
        a: Value<'ctx>,
        op: &ast::Operator,
        b: Value<'ctx>,
    ) -> Value<'ctx> {
        use rustpython_parser::ast::Operator;
        a.invoke_handler(
            ValueHandler::new()
                .handle_int(&|_, lhs_value| {
                    b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| {
                        Value::I16 {
                            value: match op {
                                Operator::Add {} => {
                                    self.builder.build_int_add(lhs_value, rhs_value, "add")
                                }
                                Operator::Sub {} => {
                                    self.builder.build_int_sub(lhs_value, rhs_value, "sub")
                                }
                                Operator::Mult => {
                                    self.builder.build_int_mul(lhs_value, rhs_value, "mul")
                                }
                                Operator::Div => {
                                    // FIXME, TODO: In Python, div int by int returns a float.
                                    unimplemented!()
                                }
                                Operator::FloorDiv => {
                                    self.builder.build_int_signed_div(lhs_value, rhs_value, "fld")
                                }
                                Operator::Mod => {
                                    self.builder.build_int_signed_rem(lhs_value, rhs_value, "mod")
                                }
                                _ => panic!(
                                    "{:?}\nNotImplemented {:?} operator for i16",
                                    self.current_source_location, op
                                ),
                            },
                        }
                    }))
                })
                .handle_float(&|_, lhs_value| {
                    b.invoke_handler(
                        ValueHandler::new().handle_float(&|_, rhs_value| Value::F32 {
                            value: match op {
                                Operator::Add {} => {
                                    self.builder.build_float_add(lhs_value, rhs_value, "add")
                                }
                                Operator::Sub {} => {
                                    self.builder.build_float_sub(lhs_value, rhs_value, "sub")
                                }
                                Operator::Mult => {
                                    self.builder.build_float_mul(lhs_value, rhs_value, "mul")
                                }
                                Operator::Div => {
                                    self.builder.build_float_div(lhs_value, rhs_value, "div")
                                }
                                Operator::FloorDiv => {
                                    // FIXME, TODO: In Python, floordiv float by float returns a int.
                                    unimplemented!()
                                }
                                Operator::Mod => {
                                    self.builder.build_float_rem(lhs_value, rhs_value, "mod")
                                }
                                _ => panic!(
                                    "{:?}\nNotImplemented {:?} operator for f32",
                                    self.current_source_location, op
                                ),
                            },
                        }),
                    )
                }),
        )
    }

    pub fn compile_comparison(
        &mut self,
        vals: &[ast::Expression],
        ops: &[ast::Comparison],
    ) -> Value<'ctx> {
        if ops.len() > 1 {
            panic!("Chained comparison is not implemented.")
        }

        if vals.len() > 2 {
            panic!("Chained comparison is not implemented.")
        }

        let a = self.compile_expr(vals.first().unwrap());
        let b = self.compile_expr(vals.last().unwrap());

        a.invoke_handler(
            ValueHandler::new()
                .handle_int(&|_, lhs_value| {
                    b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| {
                        let int_predicate = match ops.first().unwrap() {
                            ast::Comparison::Equal => IntPredicate::EQ,
                            _ => panic!("Unsupported int predicate"),
                        };
                        Value::Bool {
                            value: self.builder.build_int_compare(
                                int_predicate,
                                lhs_value,
                                rhs_value,
                                "a",
                            ),
                        }
                    }))
                })
                .handle_float(&|_, lhs_value| {
                    b.invoke_handler(ValueHandler::new().handle_float(&|_, rhs_value| {
                        let float_predicate = match ops.first().unwrap() {
                            ast::Comparison::Equal => FloatPredicate::OEQ,
                            _ => panic!("Unsupported float predicate"),
                        };
                        Value::Bool {
                            value: self.builder.build_float_compare(
                                float_predicate,
                                lhs_value,
                                rhs_value,
                                "a",
                            ),
                        }
                    }))
                }),
        )
    }

    pub fn compile(&mut self, program: ast::Program) {
        generate_prototypes(&self.module, &self.context);

        for statement in program.statements.iter() {
            if let ast::StatementType::Expression { ref expression } = statement.node {
                self.compile_expr(&expression);
            } else {
                self.compile_stmt(&statement);
            }
        }
    }
}
