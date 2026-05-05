use std::ops::Deref;

use inkwell::builder::Builder;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::float_value::FloatValue;
use crate::function::Function;
use crate::integer_value::IntegerValue;
use crate::types::Type;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    Bool(BoolValue<'ctx>),
    Function(Function<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(
        value_ir: AnyValueEnum<'ctx>,
        value_type: &Type<'ctx>,
    ) -> CompilationResult<Self> {
        Ok(match value_type {
            Type::Integer(value_type) => {
                IntegerValue::new(value_ir.into_int_value(), value_type.is_signed()).into()
            }
            Type::Float(_) => FloatValue::new(value_ir.into_float_value()).into(),
            Type::Bool(_) => BoolValue::new(value_ir.into_int_value()).into(),
            Type::Function(value_type) => {
                Function::new(value_ir.into_function_value(), value_type.clone()).into()
            }
        })
    }
}

pub trait ValueOperations<'ctx> {
    fn binary_operation(
        &self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>>;

    fn unary_operation(
        &self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<Value<'ctx>>;
}

impl<'ctx> Deref for Value<'ctx> {
    type Target = dyn ValueOperations<'ctx> + 'ctx;

    fn deref(&self) -> &Self::Target {
        match self {
            Value::Integer(value) => value,
            Value::Float(value) => value,
            Value::Bool(value) => value,
            Value::Function(value) => value,
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
