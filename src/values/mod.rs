use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::ast::Constant;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::function::Function;
use crate::types::boolean::BoolValue;
use crate::types::floating::{FloatType, FloatValue};
use crate::types::integer::{IntegerType, IntegerValue};
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
        let value = match value_type {
            Type::Integer(value_type) => {
                IntegerValue::from_ir(value_ir.into_int_value(), value_type.is_signed()).into()
            }
            Type::Float(_) => FloatValue::from_ir(value_ir.into_float_value()).into(),
            Type::Bool(_) => BoolValue::from_ir(value_ir.into_int_value()).into(),
            Type::Function(function_type) => {
                let function_ir = value_ir.into_function_value();
                Value::Function(Function::from_ir(function_ir, function_type.clone()))
            }
        };
        Ok(value)
    }

    pub fn get_type(&self) -> Type<'ctx> {
        match self {
            Value::Integer(value) => value.get_type().into(),
            Value::Float(value) => value.get_type().into(),
            Value::Bool(value) => value.get_type(),
            Value::Function(value) => Type::Function(value.get_type().clone()),
        }
    }

    pub fn do_binary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &Value<'ctx>,
    ) -> CompilationResult<()> {
        match self {
            Self::Integer(value) => {
                let other = other.to_int(builder, &value.get_type())?;
                value.do_binary_operation(builder, op, &other)
            }
            Self::Float(value) => {
                let other = other.to_float(builder, &value.get_type())?;
                value.do_binary_operation(builder, op, &other)
            }
            Self::Bool(value) => {
                let other = other.to_bool(builder)?;
                value.do_binary_operation(builder, op, &other)
            }
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn do_unary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<()> {
        match self {
            Self::Integer(value) => value.do_unary_operation(builder, op),
            Self::Float(value) => value.do_unary_operation(builder, op),
            Self::Bool(value) => value.do_unary_operation(builder, op),
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn from_constant(context: &'ctx Context, value: &Constant) -> CompilationResult<Self> {
        match value {
            Constant::Integer(value) => {
                Ok(Self::Integer(IntegerValue::from_constant(context, *value)))
            }
        }
    }

    pub fn to_int(
        &self,
        builder: &Builder<'ctx>,
        required_type: &IntegerType<'ctx>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        match self {
            Self::Integer(value) => value.promote(builder, required_type),
            Self::Bool(value) => value.to_integer(builder, required_type),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn to_float(
        &self,
        builder: &Builder<'ctx>,
        required_type: &FloatType<'ctx>,
    ) -> CompilationResult<FloatValue<'ctx>> {
        match self {
            Self::Float(value) => value.promote(builder, required_type),
            Self::Integer(value) => value.to_float(builder, required_type),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn to_bool(&self, builder: &Builder<'ctx>) -> CompilationResult<BoolValue<'ctx>> {
        match self {
            Self::Bool(value) => Ok(value.clone()),
            Self::Integer(value) => value.to_bool(builder),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn to_function(&self) -> CompilationResult<Function<'ctx>> {
        match self {
            Value::Function(value) => Ok(value.clone()),
            _ => Err(CompilationError::TypeMismatch),
        }
    }
}

impl<'ctx> TryInto<BasicValueEnum<'ctx>> for Value<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicValueEnum<'ctx>, Self::Error> {
        match self {
            Self::Integer(value) => Ok(BasicValueEnum::IntValue(value.into())),
            Self::Float(value) => Ok(BasicValueEnum::FloatValue(value.into())),
            Self::Bool(value) => Ok(BasicValueEnum::IntValue(value.into())),
            _ => Err(CompilationError::InvalidOperation),
        }
    }
}

impl<'ctx> From<Function<'ctx>> for Value<'ctx> {
    fn from(value: Function<'ctx>) -> Self {
        Value::Function(value)
    }
}
