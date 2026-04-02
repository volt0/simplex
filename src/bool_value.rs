use inkwell::values::{AnyValueEnum, IntValue};

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::integer_value::IntegerValue;
use crate::value::Value;

#[derive(Clone)]
pub struct BoolValue<'ctx> {
    pub ir: IntValue<'ctx>,
}

impl<'ctx> Into<Value<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Bool(self)
    }
}

impl<'ctx> BoolValue<'ctx> {
    pub fn from_ir(value_ir: AnyValueEnum<'ctx>) -> Self {
        if let AnyValueEnum::IntValue(value_ir) = value_ir {
            return BoolValue { ir: value_ir };
        }
        panic!("Expected BoolValue, got {:?}", value_ir);
    }

    pub fn into_ir(self) -> AnyValueEnum<'ctx> {
        AnyValueEnum::IntValue(self.ir)
    }

    pub fn to_integer(
        &self,
        value_type: Option<&IntegerType>,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        let value_type = match value_type {
            None => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I8,
            },
            Some(value_type) => value_type.clone(),
        };

        let builder = expression_translator.builder();
        let context = expression_translator.context();
        Ok(IntegerValue {
            ir: builder.build_int_z_extend(self.ir, value_type.to_ir(context), "")?,
            value_type: value_type.clone(),
        })
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs_ir = self.ir;
        let rhs_ir = match other {
            Value::Bool(other) => other.ir,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let builder = expression_translator.builder();
        Ok(BoolValue {
            ir: match operation {
                BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, "")?,
                BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, "")?,
                BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, "")?,
                _ => return Err(CompilationError::InvalidOperation),
            },
        }
        .into())
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = expression_translator.builder();
        Ok(BoolValue {
            ir: match operation {
                UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
                _ => return Err(CompilationError::InvalidOperation),
            },
        }
        .into())
    }
}
