#![allow(unused)]

use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::values::AnyValueEnum;

use crate::types::{FloatType, IntegerType};

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
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> AnyValueEnum<'ctx> {
        match self {
            Constant::Void => unimplemented!(),
            Constant::True => ctx.bool_type().const_int(1, false).into(),
            Constant::False => ctx.bool_type().const_int(0, false).into(),
            Constant::SignedInteger(int_type, value) => {
                int_type.compile(ctx).const_int(*value as u64, true).into()
            }
            Constant::UnsignedInteger(int_type, value) => {
                int_type.compile(ctx).const_int(*value, false).into()
            }
            Constant::Float(float_type, value) => {
                float_type.compile(ctx).const_float(*value).into()
            }
            Constant::String(_) => todo!(),
        }
    }
}

impl From<i64> for Constant {
    fn from(value: i64) -> Self {
        Constant::SignedInteger(IntegerType::Long, value)
    }
}
