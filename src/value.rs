use inkwell::values::BasicValueEnum;

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
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
            Type::Integer(value_type) => IntegerValue::from_ir(value_ir, value_type).into(),
            Type::Float(value_type) => FloatValue::from_ir(value_ir, value_type).into(),
            Type::Bool => BoolValue::from_ir(value_ir).into(),
        })
    }

    pub fn into_ir(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Integer(value) => value.into(),
            Value::Float(value) => value.into(),
            Value::Bool(value) => value.into(),
        }
    }

    pub fn value_type(&self) -> Type {
        match self {
            Value::Integer(value) => Type::Integer(value.value_type.clone()),
            Value::Float(value) => Type::Float(value.value_type.clone()),
            Value::Bool(_) => Type::Bool,
        }
    }

    pub fn validate_type(
        &self,
        expected_type: &Type,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        Ok(match expected_type {
            Type::Integer(expected_type) => {
                IntegerValue::from_value(self, expected_type, expression_translator)?.into()
            }
            Type::Float(expected_type) => {
                FloatValue::from_value(self, expected_type, expression_translator)?.into()
            }
            Type::Bool => self.to_bool(expression_translator)?,
        })
    }

    pub fn to_bool(
        &self,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        Ok(match self {
            Value::Integer(value) => value.to_bool(expression_translator)?.into(),
            Value::Bool(value) => Value::Bool(value.clone()),
            _ => return Err(CompilationError::TypeMismatch),
        })
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => {
                value.binary_operation(operation, other, expression_translator)
            }
            Value::Float(value) => value.binary_operation(operation, other, expression_translator),
            Value::Bool(value) => value.binary_operation(operation, other, expression_translator),
        }
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.unary_operation(operation, expression_translator),
            Value::Float(value) => value.unary_operation(operation, expression_translator),
            Value::Bool(value) => value.unary_operation(operation, expression_translator),
        }
    }
}
