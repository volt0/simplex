use inkwell::values::{AnyValueEnum, IntValue};

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::integer_value::{IntegerValue, IntegerValueType};
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

impl<'ctx> Into<IntValue<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> IntValue<'ctx> {
        self.ir
    }
}

impl<'ctx> BoolValue<'ctx> {
    pub fn new(ir: AnyValueEnum<'ctx>) -> Self {
        if let AnyValueEnum::IntValue(ir) = ir {
            return BoolValue { ir };
        }
        panic!("Expected BoolValue, got {:?}", ir);
    }

    pub fn to_integer(
        &self,
        value_type: IntegerValueType<'ctx>,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        let builder = expr_translator.builder();
        Ok(IntegerValue {
            ir: builder.build_int_z_extend(self.ir, value_type.ir, "")?,
            is_signed: value_type.is_signed,
        })
    }

    pub fn binary_operation(
        &self,
        op: BinaryOperation,
        other: &Value<'ctx>,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs_ir = self.ir;
        let rhs_ir = match other {
            Value::Bool(other) => other.ir,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let builder = expr_translator.builder();
        Ok(BoolValue {
            ir: match op {
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
        op: UnaryOperation,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = expr_translator.builder();
        Ok(BoolValue {
            ir: match op {
                UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
                _ => return Err(CompilationError::InvalidOperation),
            },
        }
        .into())
    }
}
