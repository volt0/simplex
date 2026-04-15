use inkwell::builder::Builder;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
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
    pub fn from_ir(value_ir: AnyValueEnum<'ctx>, value_type: &Type) -> CompilationResult<Self> {
        Ok(match value_type {
            Type::Integer(value_type) => {
                IntegerValue::new(value_ir.into_int_value(), value_type.is_signed()).into()
            }
            Type::Float(_) => FloatValue::new(value_ir.into_float_value()).into(),
            Type::Bool(_) => BoolValue::new(value_ir.into_int_value()).into(),
        })
    }

    pub fn binary_operation(
        self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: Self,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.binary_operation(builder, op, other),
            Value::Float(value) => value.binary_operation(builder, op, other),
            Value::Bool(value) => value.binary_operation(builder, op, other),
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn unary_operation(
        self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<Self> {
        match self {
            Value::Integer(value) => value.unary_operation(builder, op),
            Value::Float(value) => value.unary_operation(builder, op),
            Value::Bool(value) => value.unary_operation(builder, op),
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn validate_type(
        self,
        builder: &Builder<'ctx>,
        required_type: Type<'ctx>,
    ) -> CompilationResult<Self> {
        Ok(match required_type {
            Type::Integer(required_type) => required_type.validate_value(builder, self)?.into(),
            Type::Float(required_type) => required_type.validate_value(builder, self)?.into(),
            Type::Bool(_) => match self {
                Value::Bool(value) => Value::Bool(value.clone()),
                Value::Integer(value) => value.to_bool(builder)?.into(),
                _ => return Err(CompilationError::TypeMismatch),
            },
        })
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
