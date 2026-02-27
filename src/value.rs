use inkwell::values::BasicValueEnum;

use crate::expression::UnaryOperation;
use crate::integer_value::IntegerValue;

#[derive(Clone)]
pub enum Value<'ctx> {
    IntegerValue(IntegerValue<'ctx>),
}

impl<'ctx> From<IntegerValue<'ctx>> for Value<'ctx> {
    fn from(value: IntegerValue<'ctx>) -> Self {
        Value::IntegerValue(value)
    }
}

impl<'ctx> Value<'ctx> {
    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        match self {
            Value::IntegerValue(value) => value.to_ir(),
        }
    }

    pub fn unary_operation(self, operation: UnaryOperation) -> Value<'ctx> {
        match self {
            Value::IntegerValue(value) => value.unary_operation(operation).into(),
        }
    }
}
