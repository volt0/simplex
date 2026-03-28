use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;

use crate::bool_value::BoolValue;
use crate::errors::CompilationError;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_value::IntegerValue;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
    Bool(BoolValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn into_ir(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Integer(value) => value.into(),
            Value::Bool(value) => value.into(),
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
            Value::Integer(value) => {
                let other = match other {
                    Value::Integer(other) => other,
                    Value::Bool(other) => &other.to_integer(builder, context)?,
                };
                value
                    .binary_operation(operation, other, builder, context)?
                    .into()
            }
            Value::Bool(value) => {
                let other = match other {
                    Value::Bool(other) => other,
                    Value::Integer(other) => &other.to_bool(builder, context)?,
                };

                value.binary_operation(operation, other, builder)?.into()
            }
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
            Value::Bool(value) => value.unary_operation(operation, builder)?.into(),
        })
    }
}
