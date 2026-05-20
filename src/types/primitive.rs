use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::values::BasicValueEnum;

use super::boolean::{BoolTypeIR, BoolValue};
use super::floating::{FloatType, FloatValue};
use super::integer::{IntegerType, IntegerValue};
use crate::ast::Constant;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};

#[derive(Clone, PartialEq)]
pub enum PrimitiveType<'ctx> {
    Integer(IntegerType<'ctx>),
    Float(FloatType<'ctx>),
    Bool(BoolTypeIR<'ctx>),
}

impl<'ctx> PrimitiveType<'ctx> {
    pub fn check_value(
        &self,
        builder: &Builder<'ctx>,
        value: &PrimitiveValue<'ctx>,
    ) -> CompilationResult<PrimitiveValue<'ctx>> {
        Ok(match self {
            Self::Integer(required_type) => {
                PrimitiveValue::Integer(value.to_int(builder, required_type)?)
            }
            Self::Float(required_type) => {
                PrimitiveValue::Float(value.to_float(builder, required_type)?)
            }
            Self::Bool(_) => PrimitiveValue::Bool(value.to_bool(builder)?),
        })
    }
}

impl<'ctx> TryInto<BasicTypeEnum<'ctx>> for PrimitiveType<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicTypeEnum<'ctx>, Self::Error> {
        Ok(match self {
            PrimitiveType::Integer(int_type) => int_type.into(),
            PrimitiveType::Float(float_type) => float_type.into(),
            PrimitiveType::Bool(ir) => BasicTypeEnum::IntType(ir),
        })
    }
}

#[derive(Clone)]
pub enum PrimitiveValue<'ctx> {
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    Bool(BoolValue<'ctx>),
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for PrimitiveValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        match self {
            PrimitiveValue::Integer(value) => BasicValueEnum::IntValue(value.into()),
            PrimitiveValue::Float(value) => BasicValueEnum::FloatValue(value.into()),
            PrimitiveValue::Bool(value) => BasicValueEnum::IntValue(value.into()),
        }
    }
}

impl<'ctx> PrimitiveValue<'ctx> {
    pub fn from_ir(
        value_ir: BasicValueEnum<'ctx>,
        value_type: &PrimitiveType<'ctx>,
    ) -> CompilationResult<Self> {
        // TODO: Check type
        Ok(match value_type {
            PrimitiveType::Integer(value_type) => {
                IntegerValue::from_ir(value_ir.into_int_value(), value_type.is_signed()).into()
            }
            PrimitiveType::Float(_) => FloatValue::from_ir(value_ir.into_float_value()).into(),
            PrimitiveType::Bool(_) => BoolValue::from_ir(value_ir.into_int_value()).into(),
        })
    }

    pub fn from_constant(context: &'ctx Context, value: &Constant) -> Self {
        match value {
            Constant::Integer(value) => Self::Integer(IntegerValue::from_constant(context, *value)),
        }
    }

    fn to_int(
        &self,
        builder: &Builder<'ctx>,
        required_type: &IntegerType<'ctx>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        match self {
            PrimitiveValue::Integer(value) => value.promote(builder, required_type),
            PrimitiveValue::Bool(value) => value.to_integer(builder, required_type),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    fn to_float(
        &self,
        builder: &Builder<'ctx>,
        required_type: &FloatType<'ctx>,
    ) -> CompilationResult<FloatValue<'ctx>> {
        match self {
            PrimitiveValue::Float(value) => value.promote(builder, required_type),
            PrimitiveValue::Integer(value) => value.to_float(builder, required_type),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    fn to_bool(&self, builder: &Builder<'ctx>) -> CompilationResult<BoolValue<'ctx>> {
        match self {
            PrimitiveValue::Bool(value) => Ok(value.clone()),
            PrimitiveValue::Integer(value) => value.to_bool(builder),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn get_type(&self) -> PrimitiveType<'ctx> {
        match self {
            PrimitiveValue::Integer(value) => value.get_type().into(),
            PrimitiveValue::Float(value) => value.get_type().into(),
            PrimitiveValue::Bool(value) => value.get_type(),
        }
    }

    pub fn do_binary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &PrimitiveValue<'ctx>,
    ) -> CompilationResult<()> {
        match self {
            PrimitiveValue::Integer(value) => {
                let other = other.to_int(builder, &value.get_type())?;
                value.do_binary_operation(builder, op, &other)
            }
            PrimitiveValue::Float(value) => {
                let other = other.to_float(builder, &value.get_type())?;
                value.do_binary_operation(builder, op, &other)
            }
            PrimitiveValue::Bool(value) => {
                let other = other.to_bool(builder)?;
                value.do_binary_operation(builder, op, &other)
            }
        }
    }

    pub fn do_unary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<()> {
        match self {
            PrimitiveValue::Integer(value) => value.do_unary_operation(builder, op),
            PrimitiveValue::Float(value) => value.do_unary_operation(builder, op),
            PrimitiveValue::Bool(value) => value.do_unary_operation(builder, op),
        }
    }
}
