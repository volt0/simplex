use super::Value;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::function::Function;
use crate::types::primitive::PrimitiveValue;
use crate::types::Type;

#[derive(Clone)]
pub enum ValueVariant<'ctx> {
    Primitive(PrimitiveValue<'ctx>),
    Function(Function<'ctx>),
}

impl<'ctx> ValueVariant<'ctx> {
    pub fn to_primitive(&self) -> CompilationResult<PrimitiveValue<'ctx>> {
        match self {
            ValueVariant::Primitive(value) => Ok(value.clone()),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn get_type(&self) -> Type<'ctx> {
        match self {
            ValueVariant::Primitive(value) => Type::Primitive(value.get_type()),
            ValueVariant::Function(value) => Type::Function(value.get_type().clone()),
        }
    }

    pub fn do_binary_operation(
        &mut self,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
        op: BinaryOperation,
        other: &ValueVariant<'ctx>,
    ) -> CompilationResult<()> {
        match self {
            ValueVariant::Primitive(value) => {
                value.do_binary_operation(expr_translator.builder(), op, &other.to_primitive()?)
            }
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    pub fn do_unary_operation(
        &mut self,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
        op: UnaryOperation,
    ) -> CompilationResult<()> {
        match self {
            ValueVariant::Primitive(value) => {
                value.do_unary_operation(expr_translator.builder(), op)
            }
            _ => Err(CompilationError::InvalidOperation),
        }
    }
}

impl<'ctx> Into<Value<'ctx>> for ValueVariant<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value { variant: self }
    }
}
