use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum};

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

    pub fn value_type(&self) -> Type {
        match self {
            Value::Integer(value) => Type::Integer(value.value_type()),
            Value::Float(value) => Type::Float(value.value_type()),
            Value::Bool(_) => Type::Bool,
            Value::Function(_) => todo!(),
        }
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

impl<'ctx> Into<AnyValueEnum<'ctx>> for Value<'ctx> {
    fn into(self) -> AnyValueEnum<'ctx> {
        match self {
            Value::Integer(value) => AnyValueEnum::IntValue(value.into()),
            Value::Bool(value) => AnyValueEnum::IntValue(value.into()),
            Value::Float(value) => AnyValueEnum::FloatValue(value.into()),
            Value::Function(value) => AnyValueEnum::FunctionValue(value.into()),
        }
    }
}

impl<'ctx> TryInto<BasicValueEnum<'ctx>> for Value<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicValueEnum<'ctx>, Self::Error> {
        let ir: AnyValueEnum<'ctx> = self.into();
        match ir.try_into() {
            Ok(ir) => Ok(ir),
            Err(_) => Err(CompilationError::InvalidOperation),
        }
    }
}

impl<'ctx> TryInto<BasicMetadataValueEnum<'ctx>> for Value<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicMetadataValueEnum<'ctx>, Self::Error> {
        let ir: BasicValueEnum<'ctx> = self.try_into()?;
        Ok(ir.into())
    }
}
