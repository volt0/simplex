use inkwell::context::Context;

use crate::value::{IntegerValue, Value};

pub enum Expression {
    LoadConstant(Constant),
}

pub enum Constant {
    Integer(i32),
}

pub struct ExpressionTranslator<'ctx> {
    pub context: &'ctx Context,
}

impl<'ctx> ExpressionTranslator<'ctx> {
    pub fn translate(&self, expression: &Expression) -> Value<'ctx> {
        match expression {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
        }
    }

    fn translate_constant(&self, constant: &Constant) -> Value<'ctx> {
        match constant {
            Constant::Integer(value) => {
                Value::Integer(IntegerValue::from_constant(*value, self.context))
            }
        }
    }
}
