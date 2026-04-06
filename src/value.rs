use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::float_value::FloatValue;
use crate::function_value::FunctionValue;
use crate::integer_value::IntegerValue;
use crate::types::Type;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    Bool(BoolValue<'ctx>),
    Function(FunctionValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn from_any_value(
        value_ir: AnyValueEnum<'ctx>,
        value_type: &Type,
    ) -> CompilationResult<Self> {
        Ok(match value_type {
            Type::Integer(value_type) => IntegerValue::new(value_ir, value_type.is_signed).into(),
            Type::Float(_) => FloatValue::new(value_ir).into(),
            Type::Bool => BoolValue::new(value_ir).into(),
        })
    }

    pub fn validate_type(
        &self,
        expected_type: &Type,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        Ok(match expected_type {
            Type::Integer(value_type) => {
                IntegerValue::from_value(self, value_type, expr_translator)?.into()
            }
            Type::Float(value_type) => {
                FloatValue::from_value(self, value_type, expr_translator)?.into()
            }
            Type::Bool => self.to_bool(expr_translator)?,
        })
    }

    pub fn to_bool(
        &self,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        Ok(match self {
            Value::Integer(value) => value.to_bool(expr_translator)?.into(),
            Value::Bool(value) => Value::Bool(value.clone()),
            _ => return Err(CompilationError::TypeMismatch),
        })
    }

    pub fn binary_operation(
        &self,
        op: BinaryOperation,
        other: &Value<'ctx>,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.binary_operation(op, other, expr_translator),
            Value::Float(value) => value.binary_operation(op, other, expr_translator),
            Value::Bool(value) => value.binary_operation(op, other, expr_translator),
            Value::Function(_) => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn unary_operation(
        &self,
        op: UnaryOperation,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.unary_operation(op, expr_translator),
            Value::Float(value) => value.unary_operation(op, expr_translator),
            Value::Bool(value) => value.unary_operation(op, expr_translator),
            Value::Function(_) => Err(CompilationError::InvalidOperation),
        }
    }
}

impl<'ctx> TryInto<BasicValueEnum<'ctx>> for Value<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicValueEnum<'ctx>, Self::Error> {
        Ok(match self {
            Value::Integer(value) => BasicValueEnum::IntValue(value.into()),
            Value::Bool(value) => BasicValueEnum::IntValue(value.into()),
            Value::Float(value) => BasicValueEnum::FloatValue(value.into()),
            _ => return Err(CompilationError::InvalidOperation),
        })
    }
}
