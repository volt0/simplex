use std::rc::Rc;

use inkwell::context::Context as BackendContext;

use crate::types::FloatTypeSpec;
use crate::values::Value;

#[allow(unused)]
#[derive(Clone)]
pub enum Constant {
    Void,
    True,
    False,
    Integer(i32),
    Float(FloatTypeSpec, f64),
    String(Rc<str>),
}

impl Constant {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> Value<'ctx> {
        match self {
            Constant::Void => unimplemented!(),
            Constant::True => Value::from_ir(ctx.bool_type().const_int(1, false).into()),
            Constant::False => Value::from_ir(ctx.bool_type().const_int(0, false).into()),
            Constant::Integer(value) => {
                Value::new_integer(ctx.i32_type().const_int(*value as u64, true), true)
            }
            Constant::Float(float_type, value) => {
                Value::from_ir(float_type.compile(ctx).const_float(*value).into())
            }
            Constant::String(_) => todo!(),
        }
    }
}

impl From<i32> for Constant {
    fn from(value: i32) -> Self {
        Constant::Integer(value)
    }
}
