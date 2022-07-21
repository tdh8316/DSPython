use std::borrow::Borrow;

use inkwell::types::BasicTypeEnum;
use inkwell::values::{BasicValueEnum, FloatValue, IntValue, PointerValue};
use num_bigint::{BigInt, BigUint};
use num_traits::{Signed, ToPrimitive};

pub enum Value<'ctx> {
    None,
    Bool { value: IntValue<'ctx> },
    I32 { value: IntValue<'ctx> },
    F32 { value: FloatValue<'ctx> },
    Str { value: PointerValue<'ctx> },
}

impl<'ctx> Value<'ctx> {
    pub fn from_basic_value(ty: ValueType, bv: BasicValueEnum<'ctx>) -> Self {
        match bv {
            BasicValueEnum::IntValue(iv) => match ty {
                ValueType::Bool => Value::Bool { value: iv },
                ValueType::I32 => Value::I32 { value: iv },
                _ => panic!("Invalid type for int value: {:?}", ty),
            },
            BasicValueEnum::FloatValue(fv) => match ty {
                ValueType::F32 => Value::F32 { value: fv },
                _ => panic!("Invalid type for float value: {:?}", ty),
            },
            BasicValueEnum::PointerValue(pv) => match ty {
                ValueType::Str => Value::Str { value: pv },
                _ => panic!("Invalid type for pointer value: {:?}", ty),
            },
            _ => panic!("Invalid basic value: {:?}", bv),
        }
    }

    /// Experimental: This function is not verified to be correct.
    pub fn from_pointer_value(
        ty: ValueType,
        pv: PointerValue<'ctx>,
        context: &'ctx inkwell::context::Context,
    ) -> Self {
        match ty {
            ValueType::I32 => Value::I32 {
                value: pv.const_to_int(ty.to_basic_type(context).into_int_type()),
            },
            ValueType::Str => Value::Str { value: pv },
            _ => panic!("Invalid type for pointer value"),
        }
    }

    pub fn get_type(&self) -> ValueType {
        match self {
            Value::None => ValueType::None,
            Value::Bool { .. } => ValueType::Bool,
            Value::I32 { .. } => ValueType::I32,
            Value::F32 { .. } => ValueType::F32,
            Value::Str { .. } => ValueType::Str,
        }
    }

    pub fn to_basic_value(&self) -> BasicValueEnum<'ctx> {
        match self {
            Value::None => panic!("Cannot convert None to basic value"),
            Value::Bool { value } => BasicValueEnum::IntValue(*value),
            Value::I32 { value } => BasicValueEnum::IntValue(*value),
            Value::F32 { value } => BasicValueEnum::FloatValue(*value),
            Value::Str { value } => BasicValueEnum::PointerValue(*value),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum ValueType {
    None,
    Bool,
    I32,
    F32,
    Str,
}

impl ValueType {
    pub fn to_basic_type<'ctx>(
        &self,
        context: &'ctx inkwell::context::Context,
    ) -> BasicTypeEnum<'ctx> {
        match self {
            ValueType::None => panic!("Cannot convert None to basic type"),
            ValueType::Bool => BasicTypeEnum::IntType(context.bool_type()),
            ValueType::I32 => BasicTypeEnum::IntType(context.i32_type()),
            ValueType::F32 => BasicTypeEnum::FloatType(context.f32_type()),
            ValueType::Str => BasicTypeEnum::PointerType(
                context.i8_type().ptr_type(inkwell::AddressSpace::Generic),
            ),
        }
    }
}

pub fn truncate_bigint_to_u64(a: &BigInt) -> u64 {
    fn truncate_biguint_to_u64(a: &BigUint) -> u64 {
        let mask = BigUint::from(0xffff_ffff_ffff_ffffu64);
        (a & mask.borrow()).to_u64().unwrap()
    }
    let was_negative = a.is_negative();
    let abs = a.abs().to_biguint().unwrap();
    let truncated = truncate_biguint_to_u64(&abs);
    if was_negative {
        truncated.wrapping_neg()
    } else {
        truncated
    }
}
