#![allow(unused)]

use std::rc::Rc;

use inkwell::context::Context as BackendContext;

use crate::types::{FloatType, IntegerType, Type};
use crate::values::Value;

#[derive(Clone)]
pub enum Constant {
    Void,
    True,
    False,
    SignedInteger(IntegerType, i64),
    UnsignedInteger(IntegerType, u64),
    Float(FloatType, f64),
    String(Rc<str>),
}

impl Constant {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> Value<'ctx> {
        match self {
            Constant::Void => unimplemented!(),
            Constant::True => Value {
                ir: ctx.bool_type().const_int(1, false).into(),
                value_type: Type::Boolean,
            },
            Constant::False => Value {
                ir: ctx.bool_type().const_int(0, false).into(),
                value_type: Type::Boolean,
            },
            Constant::SignedInteger(int_type, value) => Value {
                ir: int_type.compile(ctx).const_int(*value as u64, true).into(),
                value_type: Type::SignedInteger(int_type.clone()),
            },
            Constant::UnsignedInteger(int_type, value) => Value {
                ir: int_type.compile(ctx).const_int(*value, false).into(),
                value_type: Type::UnsignedInteger(int_type.clone()),
            },
            Constant::Float(float_type, value) => Value {
                ir: float_type.compile(ctx).const_float(*value).into(),
                value_type: Type::Float(float_type.clone()),
            },
            Constant::String(_) => todo!(),
        }
    }
}

impl From<i64> for Constant {
    fn from(value: i64) -> Self {
        Constant::SignedInteger(IntegerType::Long, value)
    }
}
