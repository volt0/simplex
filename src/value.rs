use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;

use crate::bool_value::BoolValue;
use crate::errors::CompilationResult;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::float_value::FloatValue;
use crate::integer_value::IntegerValue;
use crate::types::Type;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    Bool(BoolValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(value_ir: BasicValueEnum<'ctx>, value_type: &Type) -> CompilationResult<Self> {
        Ok(match value_type {
            Type::Integer(value_type) => {
                Value::Integer(IntegerValue::from_ir(value_ir, value_type)?)
            }
            Type::Float(value_type) => Value::Float(FloatValue::from_ir(value_ir, value_type)?),
            Type::Bool => Value::Bool(BoolValue::from_ir(value_ir)?),
        })
    }

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
        type_hint: Option<&Type>,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => {
                value.binary_operation(operation, other, builder, context, type_hint)
            }
            Value::Float(value) => {
                value.binary_operation(operation, other, builder, context, type_hint)
            }
            Value::Bool(value) => {
                value.binary_operation(operation, other, builder, context, type_hint)
            }
        }
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
        type_hint: Option<&Type>,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.unary_operation(operation, builder, context, type_hint),
            Value::Float(value) => value.unary_operation(operation, builder, context, type_hint),
            Value::Bool(value) => value.unary_operation(operation, builder, type_hint),
        }
    }
}
