use std::collections::HashMap;

use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValue;
use inkwell::{FloatPredicate, IntPredicate};
use rustpython_parser::ast;

use crate::compiler::Compiler;
use crate::irgen::expr::CGExpr;
use crate::value::{Value, ValueHandler, ValueType};

pub trait CGStmt<'a, 'ctx> {
  fn compile_stmt(&mut self, stmt: &ast::Statement);
  fn compile_stmt_function_def(
    &mut self,
    name: &String,
    args: &Box<ast::Parameters>,
    body: &ast::Suite,
    returns: &Option<ast::Expression>,
  );
  fn compile_stmt_conditional(
    &mut self,
    cond: Value<'ctx>,
    body: &Vec<ast::Statement>,
    orelse: Option<&Vec<ast::Statement>>,
  );
  fn compile_stmt_while(
    &mut self,
    test: &ast::Expression,
    body: &ast::Suite,
    orelse: &Option<ast::Suite>,
  );
}

impl<'a, 'ctx> CGStmt<'a, 'ctx> for Compiler<'a, 'ctx> {
  fn compile_stmt(&mut self, stmt: &ast::Statement) {
    self.set_source_location(stmt.location);

    use rustpython_parser::ast::StatementType;
    match &stmt.node {
      StatementType::Expression { expression } => {
        self.compile_expr(expression);
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
          panic!("{:?}\nAsync is not supported", self.current_source_location)
        }
        let _decorators = decorator_list;
        self.compile_stmt_function_def(name, args, body, returns);
      }
      StatementType::Assign { targets, value } => {
        if targets.len() > 1 {
          panic!("Variable unpacking is not implemented")
        }

        let target = targets.first().unwrap();
        let name = match &target.node {
          ast::ExpressionType::Identifier { name } => name,
          _ => panic!(
            "{:?}\nUnsupported assign target {:?}",
            self.current_source_location, target.node
          ),
        };

        let value = self.compile_expr(value);
        let ty = value.get_type();

        if self.fn_value_opt.is_some() {
          let old_var = self.fn_scope.get(&self.fn_value()).unwrap().get(name);

          let p = if old_var.is_none() {
            self
              .builder
              .build_alloca(ty.to_basic_type(self.context), name)
          } else {
            old_var.unwrap().to_owned().1
          };
          self.builder.build_store(p, value.to_basic_value());

          // self.variables.insert(name.clone(), (ty, p));
          self
            .fn_scope
            .get_mut(&self.fn_value())
            .unwrap()
            .insert(name.clone(), (ty, p));
        } else {
          let global = self
            .module
            .add_global(ty.to_basic_type(self.context), None, name);
          global.set_unnamed_addr(true);
          global.set_initializer(&value.to_basic_value());
          let p = global.as_pointer_value();

          self.variables.insert(name.clone(), (ty, p));
        }
      }
      StatementType::Return { value } => {
        if self.fn_value_opt.is_none() {
          panic!(
            "{:?}\n'return' outside function",
            self.current_source_location
          )
        }
        if value.is_none() {
          self.builder.build_return(None);
        } else {
          let return_value = self.compile_expr(value.as_ref().unwrap());

          if return_value.get_type() == ValueType::Void {
            self.builder.build_return(None);
            return;
          }

          // Type check
          let fn_type = self
            .fn_value()
            .get_type()
            .get_return_type()
            .unwrap_or_else(|| {
              panic!(
                "{:?}\nCannot return value without function type",
                self.current_source_location
              )
            });
          let value_type = return_value.to_basic_value().get_type();
          if fn_type != value_type {
            panic!(
              "{:?}\nExpected {:?} type, but {:?}.",
              self.current_source_location, fn_type, value_type
            )
          }

          return_value.invoke_handler(
            ValueHandler::new()
              .handle_int(&|_, value| self.builder.build_return(Some(&value)))
              .handle_float(&|_, value| self.builder.build_return(Some(&value))),
          );
        }
      }
      StatementType::ImportFrom {
        level,
        module,
        names,
      } => {
        let _level = level;
        let target = module.as_ref().expect("Unknown module name");
        let _names = names;
        if target.clone() == String::from("uno") {
          // Do nothing
        } else {
          panic!("Import is not implemented");
        }
      }
      StatementType::If { test, body, orelse } => {
        let cond = self.compile_expr(test);

        match orelse {
            None /*Only if:*/ => {
                self.compile_stmt_conditional(cond, body, None);
            }
            Some(statements) => {
                self.compile_stmt_conditional(cond, body, Some(statements));
            }
        }
      }
      StatementType::While { test, body, orelse } => {
        self.compile_stmt_while(test, body, orelse);
      }
      StatementType::Pass => { /* Pass */ }
      _ => panic!(
        "{:?}\nNotImplemented statement {:?}",
        self.current_source_location, stmt.node
      ),
    }
  }

  fn compile_stmt_function_def(
    &mut self,
    name: &String,
    args: &Box<ast::Parameters>,
    body: &ast::Suite,
    returns: &Option<ast::Expression>,
  ) {
    let mut args_vec: Vec<BasicTypeEnum> = vec![];
    let mut arg_names = vec![];

    for arg in args.args.iter() {
      arg_names.push(&arg.arg);
      if arg.annotation.is_none() {
        panic!("You must provide type hint for args");
      }
      let arg_type;
      match &arg.annotation.as_ref().unwrap().node {
        ast::ExpressionType::Identifier { name } => {
          arg_type = name;
        }
        _ => {
          panic!("Unknown return annotation node");
        }
      }
      match arg_type.as_str() {
        "int" => args_vec.push(self.context.i16_type().into()),
        "float" => args_vec.push(self.context.f32_type().into()),
        _ => panic!("Unknown argument type {}", arg_type),
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
          panic!("Unknown return annotation node");
        }
      }
    }

    let f = match return_type.as_str() {
      "int8" => {
        self
          .module
          .add_function(name, self.context.i8_type().fn_type(&args_vec, false), None)
      }
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

      _ => panic!("Unknown return type {}", return_type),
    };
    let bb = self.context.append_basic_block(f, "");

    self.builder.position_at_end(bb);

    self.fn_value_opt = Some(f);
    self.fn_scope.insert(self.fn_value(), HashMap::new());

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
        panic!("NotImplemented function argument type")
      };
      let pointer = self
        .builder
        .build_alloca(v.get_type().to_basic_type(self.context), arg_names[i]);
      self.builder.build_store(pointer, bv);
      self
        .fn_scope
        .get_mut(&self.fn_value())
        .unwrap()
        .insert(arg_names[i].to_string(), (v.get_type(), pointer));
    }

    for statement in body.iter() {
      self.compile_stmt(statement);
    }

    if f.get_type().get_return_type().is_none() && bb.get_terminator().is_none() {
      self.builder.build_return(None);
    }

    self.fn_value_opt = None;
  }

  fn compile_stmt_conditional(
    &mut self,
    cond: Value<'ctx>,
    body: &Vec<ast::Statement>,
    orelse: Option<&Vec<ast::Statement>>,
  ) {
    let parent = self.fn_value();

    let then_bb = self.context.append_basic_block(parent, "then");
    let else_bb = self.context.append_basic_block(parent, "else");
    let cont_bb = self.context.append_basic_block(parent, "cont");

    let cond = cond.invoke_handler(
      ValueHandler::new()
        .handle_bool(&|_, value| value)
        // In Python, all integers except 0 are considered true.
        .handle_int(&|value, _| {
          let a = value;
          let b = Value::I16 {
            value: self.context.i16_type().const_zero(),
          };

          // This LLVM expression is same as `lhs_value != 0`
          // Therefore all integers except 0 are considered true.
          let c = a.invoke_handler(
            ValueHandler::new()
              .handle_int(&|_, lhs_value| {
                b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| Value::Bool {
                  value: self.builder.build_int_compare(
                    IntPredicate::NE,
                    lhs_value,
                    rhs_value,
                    "a",
                  ),
                }))
              })
              .handle_float(&|_, lhs_value| {
                b.invoke_handler(
                  ValueHandler::new().handle_float(&|_, rhs_value| Value::Bool {
                    value: self.builder.build_float_compare(
                      FloatPredicate::ONE,
                      lhs_value,
                      rhs_value,
                      "a",
                    ),
                  }),
                )
              }),
          );

          c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
        })
        // In Python, all float numbers except 0.0 are considered true.
        .handle_float(&|value, _| {
          let a = value;
          let b = Value::F32 {
            value: self.context.f32_type().const_zero(),
          };

          // This LLVM expression is same as `lhs_value != 0.0`
          // Therefore all float numbers except 0 are considered true.
          let c = a.invoke_handler(ValueHandler::new().handle_float(&|_, lhs_value| {
            b.invoke_handler(
              ValueHandler::new().handle_float(&|_, rhs_value| Value::Bool {
                value: self.builder.build_float_compare(
                  FloatPredicate::ONE,
                  lhs_value,
                  rhs_value,
                  "a",
                ),
              }),
            )
          }));

          c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
        }),
    );

    self
      .builder
      .build_conditional_branch(cond, then_bb, else_bb);

    self.builder.position_at_end(then_bb);
    for statement in body.iter() {
      self.compile_stmt(statement);
    }
    self.builder.build_unconditional_branch(cont_bb);

    let _then_bb = self.builder.get_insert_block().unwrap();

    self.builder.position_at_end(else_bb);

    match orelse {
      Some(statements) => {
        for statement in statements.iter() {
          self.compile_stmt(statement);
        }
      }
      None => {}
    }

    self.builder.build_unconditional_branch(cont_bb);

    let _else_bb = self.builder.get_insert_block().unwrap();

    self.builder.position_at_end(cont_bb);
  }

  fn compile_stmt_while(
    &mut self,
    test: &ast::Expression,
    body: &ast::Suite,
    orelse: &Option<ast::Suite>,
  ) {
    let parent = self.fn_value();

    let loop_bb = self.context.append_basic_block(parent, "while.body");
    let else_bb = self.context.append_basic_block(parent, "while.else");
    let after_bb = self.context.append_basic_block(parent, "while.after");

    let while_bb = self.context.append_basic_block(parent, "while");
    self.builder.build_unconditional_branch(while_bb);
    self.builder.position_at_end(while_bb);

    let start = self.compile_expr(test);
    let cond = start.invoke_handler(
      ValueHandler::new()
        .handle_bool(&|_, value| value)
        // In Python, all integers except 0 are considered true.
        .handle_int(&|value, _| {
          let a = value;
          let b = Value::I16 {
            value: self.context.i16_type().const_zero(),
          };

          // This LLVM expression is same as `lhs_value != 0`
          // Therefore all integers except 0 are considered true.
          let c = a.invoke_handler(
            ValueHandler::new()
              .handle_int(&|_, lhs_value| {
                b.invoke_handler(ValueHandler::new().handle_int(&|_, rhs_value| Value::Bool {
                  value: self.builder.build_int_compare(
                    IntPredicate::NE,
                    lhs_value,
                    rhs_value,
                    "a",
                  ),
                }))
              })
              .handle_float(&|_, lhs_value| {
                b.invoke_handler(
                  ValueHandler::new().handle_float(&|_, rhs_value| Value::Bool {
                    value: self.builder.build_float_compare(
                      FloatPredicate::ONE,
                      lhs_value,
                      rhs_value,
                      "a",
                    ),
                  }),
                )
              }),
          );

          c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
        })
        // In Python, all float numbers except 0.0 are considered true.
        .handle_float(&|value, _| {
          let a = value;
          let b = Value::F32 {
            value: self.context.f32_type().const_zero(),
          };

          // This LLVM expression is same as `lhs_value != 0.0`
          // Therefore all float numbers except 0 are considered true.
          let c = a.invoke_handler(ValueHandler::new().handle_float(&|_, lhs_value| {
            b.invoke_handler(
              ValueHandler::new().handle_float(&|_, rhs_value| Value::Bool {
                value: self.builder.build_float_compare(
                  FloatPredicate::ONE,
                  lhs_value,
                  rhs_value,
                  "a",
                ),
              }),
            )
          }));

          c.invoke_handler(ValueHandler::new().handle_bool(&|_, value| value))
        }),
    );

    self
      .builder
      .build_conditional_branch(cond, loop_bb, else_bb);
    self.builder.position_at_end(loop_bb);
    for statement in body.iter() {
      self.compile_stmt(statement);
    }
    self
      .builder
      .build_conditional_branch(cond, while_bb, else_bb);

    self.builder.position_at_end(else_bb);
    match orelse {
      Some(statements) => {
        for statement in statements.iter() {
          self.compile_stmt(statement);
        }
      }
      None => {}
    }

    self.builder.build_unconditional_branch(after_bb);
    self.builder.position_at_end(after_bb);
  }
}
