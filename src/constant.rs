use std::rc::Rc;

use inkwell::context::Context as BackendContext;

use crate::types::{FloatType, IntegerType};
use crate::values::Value;

#[allow(unused)]
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
            Constant::True => Value::from_ir(ctx.bool_type().const_int(1, false).into()),
            Constant::False => Value::from_ir(ctx.bool_type().const_int(0, false).into()),
            Constant::SignedInteger(int_type, value) => {
                Value::new_integer(int_type.compile(ctx).const_int(*value as u64, true), false)
            }
            Constant::UnsignedInteger(int_type, value) => {
                Value::new_integer(int_type.compile(ctx).const_int(*value, false), true)
            }
            Constant::Float(float_type, value) => {
                Value::from_ir(float_type.compile(ctx).const_float(*value).into())
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
