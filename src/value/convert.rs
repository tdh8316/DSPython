use std::borrow::Borrow;

use num_bigint::{BigInt, BigUint};
use num_traits::{Signed, ToPrimitive};
use rustpython_parser::ast;

pub fn truncate_bigint_to_u64(a: &BigInt) -> u64 {
    fn truncate_biguint_to_u64(a: &BigUint) -> u64 {
        use std::u64;
        let mask = BigUint::from(u64::MAX);
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

pub fn try_get_constant_string(string: &ast::StringGroup) -> Option<String> {
    fn get_constant_string_inner(out_string: &mut String, string: &ast::StringGroup) -> bool {
        match string {
            ast::StringGroup::Constant { value } => {
                out_string.push_str(&value);
                true
            }
            ast::StringGroup::Joined { values } => values
                .iter()
                .all(|value| get_constant_string_inner(out_string, value)),
            ast::StringGroup::FormattedValue { .. } => false,
        }
    }
    let mut out_string = String::new();
    if get_constant_string_inner(&mut out_string, string) {
        Some(out_string)
    } else {
        None
    }
}
