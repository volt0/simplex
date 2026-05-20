use inkwell::context::Context;
use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::ast::Constant;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::function::Function;
use crate::types::primitive::PrimitiveValue;
use crate::types::Type;

use variant::ValueVariant;

mod variant;

#[derive(Clone)]
pub struct Value<'ctx> {
    pub variant: ValueVariant<'ctx>,
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(
        value_ir: AnyValueEnum<'ctx>,
        value_type: &Type<'ctx>,
    ) -> CompilationResult<Self> {
        let variant = match value_type {
            Type::Primitive(value_type) => {
                let value = match value_ir.try_into() {
                    Ok(value_ir) => PrimitiveValue::from_ir(value_ir, value_type)?,
                    Err(_) => return Err(CompilationError::InvalidOperation),
                };
                ValueVariant::Primitive(value)
            }
            Type::Function(function_type) => {
                let function_ir = value_ir.into_function_value();
                ValueVariant::Function(Function::from_ir(function_ir, function_type.clone()))
            }
        };
        Ok(Self { variant })
    }

    pub fn from_constant(context: &'ctx Context, value: &Constant) -> CompilationResult<Self> {
        Ok(Self {
            variant: ValueVariant::Primitive(PrimitiveValue::from_constant(context, value)),
        })
    }

    pub fn to_function(&self) -> CompilationResult<Function<'ctx>> {
        match &self.variant {
            ValueVariant::Function(value) => Ok(value.clone()),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn to_primitive(&self) -> CompilationResult<PrimitiveValue<'ctx>> {
        self.variant.to_primitive()
    }

    pub fn get_type(&self) -> Type<'ctx> {
        self.variant.get_type()
    }

    pub fn do_binary_operation(
        &mut self,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
        op: BinaryOperation,
        other: &Value<'ctx>,
    ) -> CompilationResult<()> {
        self.variant
            .do_binary_operation(expr_translator, op, &other.variant)
    }

    pub fn do_unary_operation(
        &mut self,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
        op: UnaryOperation,
    ) -> CompilationResult<()> {
        self.variant.do_unary_operation(expr_translator, op)
    }
}

impl<'ctx> From<PrimitiveValue<'ctx>> for Value<'ctx> {
    fn from(value: PrimitiveValue<'ctx>) -> Self {
        Self {
            variant: ValueVariant::Primitive(value),
        }
    }
}

impl<'ctx> From<Function<'ctx>> for Value<'ctx> {
    fn from(value: Function<'ctx>) -> Self {
        Value {
            variant: ValueVariant::Function(value),
        }
    }
}

impl<'ctx> TryInto<BasicValueEnum<'ctx>> for Value<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicValueEnum<'ctx>, Self::Error> {
        match self.variant {
            ValueVariant::Primitive(value) => Ok(value.into()),
            _ => Err(CompilationError::InvalidOperation),
        }
    }
}
