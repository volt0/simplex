use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;

use crate::bool_value::BoolValue;
use crate::errors::CompilationResult;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::float_value::FloatValue;
use crate::integer_value::IntegerValue;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    Bool(BoolValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn into_ir(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Integer(value) => value.into(),
            Value::Float(value) => value.into(),
            Value::Bool(value) => value.into(),
        }
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.binary_operation(operation, other, builder, context),
            Value::Float(value) => value.binary_operation(operation, other, builder, context),
            Value::Bool(value) => value.binary_operation(operation, other, builder, context),
        }
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.unary_operation(operation, builder, context),
            Value::Float(value) => value.unary_operation(operation, builder),
            Value::Bool(value) => value.unary_operation(operation, builder),
        }
    }
}
