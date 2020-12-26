#![allow(warnings)]

use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum, FloatType, IntType, PointerType, VoidType};
use inkwell::values::{AnyValueEnum, BasicValueEnum, FloatValue, IntValue, PointerValue, ArrayValue};
use inkwell::AddressSpace;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Value<'ctx> {
    Void,
    Array { value: ArrayValue<'ctx>},
    Bool { value: IntValue<'ctx> },
    I8 { value: IntValue<'ctx> },
    I16 { value: IntValue<'ctx> },
    I32 { value: IntValue<'ctx> },
    I64 { value: IntValue<'ctx> },
    I128 { value: IntValue<'ctx> },
    U8 { value: IntValue<'ctx> },
    U16 { value: IntValue<'ctx> },
    U32 { value: IntValue<'ctx> },
    U64 { value: IntValue<'ctx> },
    U128 { value: IntValue<'ctx> },
    F16 { value: FloatValue<'ctx> },
    F32 { value: FloatValue<'ctx> },
    F64 { value: FloatValue<'ctx> },
    Str { value: PointerValue<'ctx> },
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ValueType {
    Void,
    Array,
    Bool,
    I8,
    I16,
    I32,
    I64,
    I128,
    U8,
    U16,
    U32,
    U64,
    U128,
    F16,
    F32,
    F64,
    Str,
}

#[derive(Debug, PartialEq)]
pub enum ValueTypeGroup {
    Void,
    Array,
    Bool,
    Int,
    UInt,
    Float,
    Str,
}

pub struct ValueHandler<'cb, 'ctx: 'cb, T> {
    void_handler: &'cb dyn Fn(&Value<'ctx>) -> T,
    array_handler: &'cb dyn Fn(&Value<'ctx>, ArrayValue<'ctx>) -> T,
    bool_handler: &'cb dyn Fn(&Value<'ctx>, IntValue<'ctx>) -> T,
    int_handler: &'cb dyn Fn(&Value<'ctx>, IntValue<'ctx>) -> T,
    unsigned_int_handler: &'cb dyn Fn(&Value<'ctx>, IntValue<'ctx>) -> T,
    float_handler: &'cb dyn Fn(&Value<'ctx>, FloatValue<'ctx>) -> T,
    str_handler: &'cb dyn Fn(&Value<'ctx>, PointerValue<'ctx>) -> T,
}

pub struct ValueTypeHandler<'cb, 'ctx: 'cb, T> {
    void_handler: &'cb dyn Fn(&ValueType, VoidType<'ctx>) -> T,
    array_handler: &'cb dyn Fn(&ValueType, ArrayValue<'ctx>) -> T,
    bool_handler: &'cb dyn Fn(&ValueType, IntType<'ctx>) -> T,
    int_handler: &'cb dyn Fn(&ValueType, IntType<'ctx>) -> T,
    unsigned_int_handler: &'cb dyn Fn(&ValueType, IntType<'ctx>) -> T,
    float_handler: &'cb dyn Fn(&ValueType, FloatType<'ctx>) -> T,
    str_handler: &'cb dyn Fn(&ValueType, PointerType<'ctx>) -> T,
}

impl<'cb, 'ctx: 'cb, T> ValueHandler<'cb, 'ctx, T> {
    pub fn new() -> ValueHandler<'cb, 'ctx, T> {
        ValueHandler {
            void_handler: &|_| panic!("wrong type; void type is not allowed."),
            array_handler: &|_| panic!("wrong type; array type is not allowed."),
            bool_handler: &|_, _| panic!("wrong type; bool type is not allowed."),
            int_handler: &|_, _| panic!("wrong type; int type is not allowed."),
            unsigned_int_handler: &|_, _| panic!("wrong type; unsigned int type is not allowed."),
            float_handler: &|_, _| panic!("wrong type; float type is not allowed."),
            str_handler: &|_, _| panic!("wrong type; str type is not allowed."),
        }
    }

    pub fn handle_void(&mut self, handler: &'cb dyn Fn(&Value<'ctx>) -> T) -> &mut Self {
        self.void_handler = handler;
        self
    }

    pub fn handle_bool(
        &mut self,
        handler: &'cb dyn Fn(&Value<'ctx>, IntValue<'ctx>) -> T,
    ) -> &mut Self {
        self.bool_handler = handler;
        self
    }

    pub fn handle_int(
        &mut self,
        handler: &'cb dyn Fn(&Value<'ctx>, IntValue<'ctx>) -> T,
    ) -> &mut Self {
        self.int_handler = handler;
        self
    }

    pub fn handle_unsigned_int(
        &mut self,
        handler: &'cb dyn Fn(&Value<'ctx>, IntValue<'ctx>) -> T,
    ) -> &mut Self {
        self.unsigned_int_handler = handler;
        self
    }

    pub fn handle_float(
        &mut self,
        handler: &'cb dyn Fn(&Value<'ctx>, FloatValue<'ctx>) -> T,
    ) -> &mut Self {
        self.float_handler = handler;
        self
    }

    pub fn handle_str(
        &mut self,
        handler: &'cb dyn Fn(&Value<'ctx>, PointerValue<'ctx>) -> T,
    ) -> &mut Self {
        self.str_handler = handler;
        self
    }
}

impl<'cb, 'ctx: 'cb, T> ValueTypeHandler<'cb, 'ctx, T> {
    pub fn new() -> ValueTypeHandler<'cb, 'ctx, T> {
        ValueTypeHandler {
            void_handler: &|_, _| panic!("wrong type; void type is not allowed."),
            array_handler: &|_, _| panic!("wrong type; array type is not allowed."),
            bool_handler: &|_, _| panic!("wrong type; bool type is not allowed."),
            int_handler: &|_, _| panic!("wrong type; int type is not allowed."),
            unsigned_int_handler: &|_, _| panic!("wrong type; unsigned int type is not allowed."),
            float_handler: &|_, _| panic!("wrong type; float type is not allowed."),
            str_handler: &|_, _| panic!("wrong type; str type is not allowed."),
        }
    }

    pub fn handle_void(
        &mut self,
        handler: &'cb dyn Fn(&ValueType, VoidType<'ctx>) -> T,
    ) -> &mut Self {
        self.void_handler = handler;
        self
    }

    pub fn handle_bool(
        &mut self,
        handler: &'cb dyn Fn(&ValueType, IntType<'ctx>) -> T,
    ) -> &mut Self {
        self.bool_handler = handler;
        self
    }

    pub fn handle_int(
        &mut self,
        handler: &'cb dyn Fn(&ValueType, IntType<'ctx>) -> T,
    ) -> &mut Self {
        self.int_handler = handler;
        self
    }

    pub fn handle_unsigned_int(
        &mut self,
        handler: &'cb dyn Fn(&ValueType, IntType<'ctx>) -> T,
    ) -> &mut Self {
        self.unsigned_int_handler = handler;
        self
    }

    pub fn handle_float(
        &mut self,
        handler: &'cb dyn Fn(&ValueType, FloatType<'ctx>) -> T,
    ) -> &mut Self {
        self.float_handler = handler;
        self
    }

    pub fn handle_str(
        &mut self,
        handler: &'cb dyn Fn(&ValueType, PointerType<'ctx>) -> T,
    ) -> &mut Self {
        self.str_handler = handler;
        self
    }
}

impl<'ctx> Value<'ctx> {
    pub fn get_type(&self) -> ValueType {
        match self {
            Value::Void => ValueType::Void,
            Value::Array {value: _} => ValueType::Array,
            Value::Bool { value: _ } => ValueType::Bool,
            Value::I8 { value: _ } => ValueType::I8,
            Value::I16 { value: _ } => ValueType::I16,
            Value::I32 { value: _ } => ValueType::I32,
            Value::I64 { value: _ } => ValueType::I64,
            Value::I128 { value: _ } => ValueType::I128,
            Value::U8 { value: _ } => ValueType::U8,
            Value::U16 { value: _ } => ValueType::U16,
            Value::U32 { value: _ } => ValueType::U32,
            Value::U64 { value: _ } => ValueType::U64,
            Value::U128 { value: _ } => ValueType::U128,
            Value::F16 { value: _ } => ValueType::F16,
            Value::F32 { value: _ } => ValueType::F32,
            Value::F64 { value: _ } => ValueType::F64,
            Value::Str { value: _ } => ValueType::Str,
        }
    }

    pub fn from_int_value(bitwidth: usize, value: IntValue<'ctx>) -> Value<'ctx> {
        match bitwidth {
            8 => Value::I8 { value },
            16 => Value::I16 { value },
            32 => Value::I32 { value },
            64 => Value::I64 { value },
            128 => Value::I128 { value },
            _ => unreachable!(),
        }
    }

    pub fn from_unsigned_int_value(bitwidth: usize, value: IntValue<'ctx>) -> Value<'ctx> {
        match bitwidth {
            8 => Value::U8 { value },
            16 => Value::U16 { value },
            32 => Value::U32 { value },
            64 => Value::U64 { value },
            128 => Value::U128 { value },
            _ => unreachable!(),
        }
    }

    pub fn from_float_value(bitwidth: usize, value: FloatValue<'ctx>) -> Value<'ctx> {
        match bitwidth {
            16 => Value::F16 { value },
            32 => Value::F32 { value },
            64 => Value::F64 { value },
            _ => unreachable!(),
        }
    }

    pub fn from_any_value(value_type: ValueType, any_value: AnyValueEnum<'ctx>) -> Value<'ctx> {
        match any_value {
            AnyValueEnum::IntValue(value) => match value_type {
                ValueType::Bool => Value::Bool { value },
                ValueType::I8 => Value::I8 { value },
                ValueType::I16 => Value::I16 { value },
                ValueType::I32 => Value::I32 { value },
                ValueType::I64 => Value::I64 { value },
                ValueType::I128 => Value::I128 { value },
                ValueType::U8 => Value::U8 { value },
                ValueType::U16 => Value::U16 { value },
                ValueType::U32 => Value::U32 { value },
                ValueType::U64 => Value::U64 { value },
                ValueType::U128 => Value::U128 { value },
                _ => panic!(
                    "value type mismatch; given value is not instance of {:?}",
                    value_type
                ),
            },
            AnyValueEnum::FloatValue(value) => match value_type {
                ValueType::F16 => Value::F16 { value },
                ValueType::F32 => Value::F32 { value },
                ValueType::F64 => Value::F64 { value },
                _ => panic!(
                    "value type mismatch; given value is not instance of {:?}",
                    value_type
                ),
            },
            AnyValueEnum::PointerValue(value) => match value_type {
                ValueType::Str => Value::Str { value },
                _ => panic!(
                    "value type mismatch; given value is not instance of {:?}",
                    value_type
                ),
            },
            AnyValueEnum::PhiValue(value) => {
                Value::from_basic_value(value_type, value.as_basic_value())
            }
            _ => panic!("unexpected type encountered"),
        }
    }

    pub fn from_basic_value(
        value_type: ValueType,
        basic_value: BasicValueEnum<'ctx>,
    ) -> Value<'ctx> {
        match basic_value {
            BasicValueEnum::IntValue(value) => match value_type {
                ValueType::Bool => Value::Bool { value },
                ValueType::I8 => Value::I8 { value },
                ValueType::I16 => Value::I16 { value },
                ValueType::I32 => Value::I32 { value },
                ValueType::I64 => Value::I64 { value },
                ValueType::I128 => Value::I128 { value },
                ValueType::U8 => Value::U8 { value },
                ValueType::U16 => Value::U16 { value },
                ValueType::U32 => Value::U32 { value },
                ValueType::U64 => Value::U64 { value },
                ValueType::U128 => Value::U128 { value },
                _ => panic!(
                    "value type mismatch; given value is not instance of {:?}",
                    value_type
                ),
            },
            BasicValueEnum::FloatValue(value) => match value_type {
                ValueType::F16 => Value::F16 { value },
                ValueType::F32 => Value::F32 { value },
                ValueType::F64 => Value::F64 { value },
                _ => panic!(
                    "value type mismatch; given value is not instance of {:?}",
                    value_type
                ),
            },
            BasicValueEnum::PointerValue(value) => match value_type {
                ValueType::Str => Value::Str { value },
                _ => panic!(
                    "value type mismatch; given value is not instance of {:?}",
                    value_type
                ),
            },
            _ => panic!("unexpected type encountered"),
        }
    }

    pub fn to_any_value(&self) -> AnyValueEnum<'ctx> {
        match self {
            Value::Void => panic!("void is not acceptible"),
            Value::Array { value } => AnyValueEnum::ArrayValue(*value),
            Value::Bool { value } => AnyValueEnum::IntValue(*value),
            Value::I8 { value } => AnyValueEnum::IntValue(*value),
            Value::I16 { value } => AnyValueEnum::IntValue(*value),
            Value::I32 { value } => AnyValueEnum::IntValue(*value),
            Value::I64 { value } => AnyValueEnum::IntValue(*value),
            Value::I128 { value } => AnyValueEnum::IntValue(*value),
            Value::U8 { value } => AnyValueEnum::IntValue(*value),
            Value::U16 { value } => AnyValueEnum::IntValue(*value),
            Value::U32 { value } => AnyValueEnum::IntValue(*value),
            Value::U64 { value } => AnyValueEnum::IntValue(*value),
            Value::U128 { value } => AnyValueEnum::IntValue(*value),
            Value::F16 { value } => AnyValueEnum::FloatValue(*value),
            Value::F32 { value } => AnyValueEnum::FloatValue(*value),
            Value::F64 { value } => AnyValueEnum::FloatValue(*value),
            Value::Str { value } => AnyValueEnum::PointerValue(*value),
        }
    }

    pub fn to_basic_value(&self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Void => panic!("void is not acceptible"),
            Value::Array { value } => BasicValueEnum::ArrayValue(*value),
            Value::Bool { value } => BasicValueEnum::IntValue(*value),
            Value::I8 { value } => BasicValueEnum::IntValue(*value),
            Value::I16 { value } => BasicValueEnum::IntValue(*value),
            Value::I32 { value } => BasicValueEnum::IntValue(*value),
            Value::I64 { value } => BasicValueEnum::IntValue(*value),
            Value::I128 { value } => BasicValueEnum::IntValue(*value),
            Value::U8 { value } => BasicValueEnum::IntValue(*value),
            Value::U16 { value } => BasicValueEnum::IntValue(*value),
            Value::U32 { value } => BasicValueEnum::IntValue(*value),
            Value::U64 { value } => BasicValueEnum::IntValue(*value),
            Value::U128 { value } => BasicValueEnum::IntValue(*value),
            Value::F16 { value } => BasicValueEnum::FloatValue(*value),
            Value::F32 { value } => BasicValueEnum::FloatValue(*value),
            Value::F64 { value } => BasicValueEnum::FloatValue(*value),
            Value::Str { value } => BasicValueEnum::PointerValue(*value),
        }
    }

    pub fn invoke_handler<'cb, T>(&self, value_handler: &mut ValueHandler<'cb, 'ctx, T>) -> T {
        match self {
            Value::Void => (*value_handler.void_handler)(self),
            Value::Array { value } => (*value_handler.arr)(self, *value),
            Value::Bool { value } => (*value_handler.bool_handler)(self, *value),
            Value::I8 { value } => (*value_handler.int_handler)(self, *value),
            Value::I16 { value } => (*value_handler.int_handler)(self, *value),
            Value::I32 { value } => (*value_handler.int_handler)(self, *value),
            Value::I64 { value } => (*value_handler.int_handler)(self, *value),
            Value::I128 { value } => (*value_handler.int_handler)(self, *value),
            Value::U8 { value } => (*value_handler.unsigned_int_handler)(self, *value),
            Value::U16 { value } => (*value_handler.unsigned_int_handler)(self, *value),
            Value::U32 { value } => (*value_handler.unsigned_int_handler)(self, *value),
            Value::U64 { value } => (*value_handler.unsigned_int_handler)(self, *value),
            Value::U128 { value } => (*value_handler.unsigned_int_handler)(self, *value),
            Value::F16 { value } => (*value_handler.float_handler)(self, *value),
            Value::F32 { value } => (*value_handler.float_handler)(self, *value),
            Value::F64 { value } => (*value_handler.float_handler)(self, *value),
            Value::Str { value } => (*value_handler.str_handler)(self, *value),
        }
    }
}

impl ValueType {
    pub fn is_void(&self) -> bool {
        match self {
            ValueType::Void => true,
            _ => false,
        }
    }

    pub fn to_any_type<'ctx>(&self, context: &'ctx Context) -> AnyTypeEnum<'ctx> {
        match self {
            ValueType::Void => AnyTypeEnum::VoidType(context.void_type()),
            ValueType::Array => unimplemented!(),
            ValueType::Bool => AnyTypeEnum::IntType(context.bool_type()),
            ValueType::I8 => AnyTypeEnum::IntType(context.i8_type()),
            ValueType::I16 => AnyTypeEnum::IntType(context.i16_type()),
            ValueType::I32 => AnyTypeEnum::IntType(context.i32_type()),
            ValueType::I64 => AnyTypeEnum::IntType(context.i64_type()),
            ValueType::I128 => AnyTypeEnum::IntType(context.i128_type()),
            ValueType::U8 => AnyTypeEnum::IntType(context.i8_type()),
            ValueType::U16 => AnyTypeEnum::IntType(context.i16_type()),
            ValueType::U32 => AnyTypeEnum::IntType(context.i32_type()),
            ValueType::U64 => AnyTypeEnum::IntType(context.i64_type()),
            ValueType::U128 => AnyTypeEnum::IntType(context.i128_type()),
            ValueType::F16 => AnyTypeEnum::FloatType(context.f16_type()),
            ValueType::F32 => AnyTypeEnum::FloatType(context.f32_type()),
            ValueType::F64 => AnyTypeEnum::FloatType(context.f64_type()),
            ValueType::Str => {
                AnyTypeEnum::PointerType(context.i8_type().ptr_type(AddressSpace::Generic))
            }
        }
    }

    pub fn to_basic_type<'ctx>(&self, context: &'ctx Context) -> BasicTypeEnum<'ctx> {
        match self {
            ValueType::Void => panic!("void is not acceptible"),
            ValueType::Array => unimplemented!(),
            ValueType::Bool => BasicTypeEnum::IntType(context.bool_type()),
            ValueType::I8 => BasicTypeEnum::IntType(context.i8_type()),
            ValueType::I16 => BasicTypeEnum::IntType(context.i16_type()),
            ValueType::I32 => BasicTypeEnum::IntType(context.i32_type()),
            ValueType::I64 => BasicTypeEnum::IntType(context.i64_type()),
            ValueType::I128 => BasicTypeEnum::IntType(context.i128_type()),
            ValueType::U8 => BasicTypeEnum::IntType(context.i8_type()),
            ValueType::U16 => BasicTypeEnum::IntType(context.i16_type()),
            ValueType::U32 => BasicTypeEnum::IntType(context.i32_type()),
            ValueType::U64 => BasicTypeEnum::IntType(context.i64_type()),
            ValueType::U128 => BasicTypeEnum::IntType(context.i128_type()),
            ValueType::F16 => BasicTypeEnum::FloatType(context.f16_type()),
            ValueType::F32 => BasicTypeEnum::FloatType(context.f32_type()),
            ValueType::F64 => BasicTypeEnum::FloatType(context.f64_type()),
            ValueType::Str => {
                BasicTypeEnum::PointerType(context.i8_type().ptr_type(AddressSpace::Generic))
            }
        }
    }

    pub fn get_group(&self) -> ValueTypeGroup {
        match self {
            ValueType::Void => ValueTypeGroup::Void,
            ValueType::Array => ValueTypeGroup::Array,
            ValueType::Bool => ValueTypeGroup::Bool,
            ValueType::I8 => ValueTypeGroup::Int,
            ValueType::I16 => ValueTypeGroup::Int,
            ValueType::I32 => ValueTypeGroup::Int,
            ValueType::I64 => ValueTypeGroup::Int,
            ValueType::I128 => ValueTypeGroup::Int,
            ValueType::U8 => ValueTypeGroup::UInt,
            ValueType::U16 => ValueTypeGroup::UInt,
            ValueType::U32 => ValueTypeGroup::UInt,
            ValueType::U64 => ValueTypeGroup::UInt,
            ValueType::U128 => ValueTypeGroup::UInt,
            ValueType::F16 => ValueTypeGroup::Float,
            ValueType::F32 => ValueTypeGroup::Float,
            ValueType::F64 => ValueTypeGroup::Float,
            ValueType::Str => ValueTypeGroup::Str,
        }
    }

    pub fn get_bitwidth(&self) -> usize {
        match self {
            ValueType::Void => 0,
            ValueType::Array => 0, // Unknown
            ValueType::Bool => 1,
            ValueType::I8 => 8,
            ValueType::I16 => 16,
            ValueType::I32 => 32,
            ValueType::I64 => 64,
            ValueType::I128 => 128,
            ValueType::U8 => 8,
            ValueType::U16 => 16,
            ValueType::U32 => 32,
            ValueType::U64 => 64,
            ValueType::U128 => 128,
            ValueType::F16 => 16,
            ValueType::F32 => 32,
            ValueType::F64 => 64,
            ValueType::Str => 0, //Unknown
        }
    }

    pub fn invoke_handler<'cb, 'ctx: 'cb, T>(
        &self,
        context: &'ctx Context,
        value_type_handler: &mut ValueTypeHandler<'cb, 'ctx, T>,
    ) -> T {
        match self {
            ValueType::Void => (*value_type_handler.void_handler)(self, context.void_type()),
            ValueType::Array => unimplemented!(),
            ValueType::Bool => (*value_type_handler.bool_handler)(self, context.bool_type()),
            ValueType::I8 => (*value_type_handler.int_handler)(self, context.i8_type()),
            ValueType::I16 => (*value_type_handler.int_handler)(self, context.i16_type()),
            ValueType::I32 => (*value_type_handler.int_handler)(self, context.i32_type()),
            ValueType::I64 => (*value_type_handler.int_handler)(self, context.i64_type()),
            ValueType::I128 => (*value_type_handler.int_handler)(self, context.i128_type()),
            ValueType::U8 => (*value_type_handler.unsigned_int_handler)(self, context.i8_type()),
            ValueType::U16 => (*value_type_handler.unsigned_int_handler)(self, context.i16_type()),
            ValueType::U32 => (*value_type_handler.unsigned_int_handler)(self, context.i32_type()),
            ValueType::U64 => (*value_type_handler.unsigned_int_handler)(self, context.i64_type()),
            ValueType::U128 => {
                (*value_type_handler.unsigned_int_handler)(self, context.i128_type())
            }
            ValueType::F16 => (*value_type_handler.float_handler)(self, context.f16_type()),
            ValueType::F32 => (*value_type_handler.float_handler)(self, context.f32_type()),
            ValueType::F64 => (*value_type_handler.float_handler)(self, context.f64_type()),
            ValueType::Str => (*value_type_handler.str_handler)(
                self,
                context.i8_type().ptr_type(AddressSpace::Generic),
            ),
        }
    }

    pub fn merge_group<'a>(lhs: &'a ValueType, rhs: &'a ValueType) -> Option<&'a ValueType> {
        if lhs.get_group() != rhs.get_group() {
            Option::None
        } else {
            Option::Some(if lhs.get_bitwidth() < rhs.get_bitwidth() {
                rhs
            } else {
                lhs
            })
        }
    }
}
