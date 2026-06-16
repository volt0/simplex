use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::ast::Constant;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::function::Function;
use crate::types::boolean::BoolValue;
use crate::types::floating::FloatValue;
use crate::types::integer::IntValue;
use crate::types::Type;

#[derive(Clone)]
pub enum Value<'ctx> {
    Int(IntValue<'ctx>),
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
            Type::Int(value_type) => {
                let value_ir = value_ir.into_int_value();
                IntValue::from_ir(value_ir, value_type.is_signed()).into()
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

    pub fn from_constant(context: &'ctx Context, value: &Constant) -> CompilationResult<Self> {
        match value {
            Constant::Int(value) => Ok(Self::Int(IntValue::from_constant(context, *value))),
        }
    }

    #[allow(unused)]
    pub fn get_type(&self) -> Type<'ctx> {
        match self {
            Value::Int(value) => value.get_type().into(),
            Value::Float(value) => value.get_type().into(),
            Value::Bool(value) => value.get_type().into(),
            Value::Function(value) => Type::Function(value.get_type().clone()),
        }
    }

    pub fn do_binary_operation(
        &self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        match self {
            Self::Int(value) => value.do_binary_operation(builder, op, &other),
            Self::Float(value) => value.do_binary_operation(builder, op, &other),
            Self::Bool(value) => value.do_binary_operation(builder, op, &other),
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn do_unary_operation(
        &self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<Value<'ctx>> {
        match self {
            Self::Int(value) => value.do_unary_operation(builder, op),
            Self::Float(value) => value.do_unary_operation(builder, op),
            Self::Bool(value) => value.do_unary_operation(builder, op),
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn do_call(
        &self,
        builder: &Builder<'ctx>,
        args: &[Value<'ctx>],
    ) -> CompilationResult<Value<'ctx>> {
        match self {
            Value::Function(function) => function.do_call(builder, args),
            _ => Err(CompilationError::TypeMismatch),
        }
    }
}

impl<'ctx> TryInto<BasicValueEnum<'ctx>> for Value<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicValueEnum<'ctx>, Self::Error> {
        match self {
            Self::Int(value) => Ok(BasicValueEnum::IntValue(value.into())),
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
