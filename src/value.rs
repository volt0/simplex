use inkwell::context::Context;
use inkwell::values::{AnyValueEnum, BasicMetadataValueEnum, BasicValueEnum};

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::float_value::{FloatValue, FloatValueType};
use crate::function_value::FunctionValue;
use crate::integer_value::{IntegerValue, IntegerValueType};
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
        value_type: ValueType<'ctx>,
    ) -> CompilationResult<Self> {
        Ok(match value_type {
            ValueType::Integer(value_type) => {
                IntegerValue::new(value_ir, value_type.is_signed).into()
            }
            ValueType::Float(_) => FloatValue::new(value_ir).into(),
            ValueType::Bool => BoolValue::new(value_ir).into(),
        })
    }

    pub fn value_type(&self) -> Type {
        match self {
            Value::Integer(value) => Type::Integer(value.type_of().into()),
            Value::Float(value) => Type::Float(value.type_of().into()),
            Value::Bool(_) => Type::Bool,
            Value::Function(_) => todo!(),
        }
    }

    pub fn validate_type(
        &self,
        expected_type: ValueType<'ctx>,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        Ok(match expected_type {
            ValueType::Integer(value_type) => {
                IntegerValue::from_value(self, value_type, expr_translator)?.into()
            }
            ValueType::Float(value_type) => {
                FloatValue::from_value(self, value_type, expr_translator)?.into()
            }
            ValueType::Bool => self.to_bool(expr_translator)?,
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

#[derive(Clone)]
pub enum ValueType<'ctx> {
    Integer(IntegerValueType<'ctx>),
    Float(FloatValueType<'ctx>),
    Bool,
}

impl<'ctx> ValueType<'ctx> {
    pub fn new(type_spec: &Type, context: &'ctx Context) -> Self {
        match type_spec {
            Type::Integer(type_spec) => {
                ValueType::Integer(IntegerValueType::new(type_spec, context))
            }
            Type::Float(type_spec) => ValueType::Float(FloatValueType::new(type_spec, context)),
            Type::Bool => ValueType::Bool,
        }
    }
}
