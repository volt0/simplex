use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;

use crate::errors::CompilationError;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_value::IntegerValue;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn into_ir(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Integer(value) => value.into(),
        }
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Result<Self, CompilationError> {
        Ok(match self {
            Value::Integer(value) => match other {
                Value::Integer(other) => value
                    .binary_operation(operation, other, builder, context)?
                    .into(),
            },
        })
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Result<Self, CompilationError> {
        Ok(match self {
            Value::Integer(value) => value.unary_operation(operation, builder, context)?.into(),
        })
    }
}
