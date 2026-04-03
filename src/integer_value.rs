use inkwell::values::{AnyValueEnum, IntValue};
use inkwell::IntPredicate;

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::float_type::FloatType;
use crate::float_value::FloatValue;
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::value::Value;

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub ir: IntValue<'ctx>,
    pub is_signed: bool,
}

impl<'ctx> Into<Value<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Integer(self)
    }
}

impl<'ctx> Into<IntValue<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> IntValue<'ctx> {
        self.ir
    }
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn new(ir: AnyValueEnum<'ctx>, is_signed: bool) -> Self {
        match ir {
            AnyValueEnum::IntValue(ir) => IntegerValue { ir, is_signed },
            _ => panic!("Expected IntValue, got {:?}", ir),
        }
    }

    pub fn from_value(
        value: &Value<'ctx>,
        expr_type: &IntegerType,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        Ok(match value {
            Value::Integer(value) => value.extend_to(expr_type, expr_translator)?,
            Value::Bool(value) => value.to_integer(Some(expr_type), expr_translator)?,
            _ => return Err(CompilationError::TypeMismatch),
        })
    }

    pub fn from_constant(
        value: i32,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> Self {
        let context = expr_translator.context();
        IntegerValue {
            ir: context.i32_type().const_int(value as u64, true),
            is_signed: true,
        }
    }

    pub fn to_bool(
        &self,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<BoolValue<'ctx>> {
        let builder = expr_translator.builder();
        Ok(BoolValue {
            ir: builder.build_int_compare(
                IntPredicate::NE,
                self.ir,
                self.ir.get_type().const_int(0, false),
                "",
            )?,
        })
    }

    pub fn to_float(
        &self,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<FloatValue<'ctx>> {
        let result_type = match self.value_type().width {
            IntegerTypeSize::I8 | IntegerTypeSize::I16 => FloatType::F32,
            IntegerTypeSize::I32 => FloatType::F64,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let builder = expr_translator.builder();
        let context = expr_translator.context();
        let result_ir = if self.is_signed {
            builder.build_signed_int_to_float(self.ir, result_type.to_ir(context), "")?
        } else {
            builder.build_unsigned_int_to_float(self.ir, result_type.to_ir(context), "")?
        };

        Ok(FloatValue { ir: result_ir }.into())
    }

    pub fn value_type(&self) -> IntegerType {
        let value_type_ir = self.ir.get_type();
        IntegerType {
            width: match value_type_ir.get_bit_width() {
                8 => IntegerTypeSize::I8,
                16 => IntegerTypeSize::I16,
                32 => IntegerTypeSize::I32,
                64 => IntegerTypeSize::I64,
                width => panic!("Invalid integer type width: {}", width),
            },
            is_signed: self.is_signed,
        }
    }

    pub fn binary_operation(
        &self,
        op: BinaryOperation,
        other: &Value<'ctx>,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs_ir = self.ir;
        let rhs_ir = match other {
            Value::Integer(other) => other.ir,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let builder = expr_translator.builder();
        let result_ir = match op {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, ""),
            BinaryOperation::Sub => builder.build_int_sub(lhs_ir, rhs_ir, ""),
            BinaryOperation::Mul => builder.build_int_mul(lhs_ir, rhs_ir, ""),
            BinaryOperation::Div => {
                if self.is_signed {
                    builder.build_int_signed_div(lhs_ir, rhs_ir, "")
                } else {
                    builder.build_int_unsigned_div(lhs_ir, rhs_ir, "")
                }
            }
            BinaryOperation::Mod => {
                if self.is_signed {
                    builder.build_int_signed_rem(lhs_ir, rhs_ir, "")
                } else {
                    builder.build_int_unsigned_rem(lhs_ir, rhs_ir, "")
                }
            }
            BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, ""),
            BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, ""),
            BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, ""),
            BinaryOperation::ShiftLeft => builder.build_left_shift(lhs_ir, rhs_ir, ""),
            BinaryOperation::ShiftRight => {
                builder.build_right_shift(lhs_ir, rhs_ir, self.is_signed, "")
            }
        };

        Ok(IntegerValue {
            ir: result_ir?,
            is_signed: self.is_signed,
        }
        .into())
    }

    pub fn unary_operation(
        &self,
        op: UnaryOperation,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = expr_translator.builder();
        let result_ir = match op {
            UnaryOperation::Plus => self.ir,
            UnaryOperation::Minus => builder.build_int_neg(self.ir, "")?,
            UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
        };

        Ok(IntegerValue {
            ir: result_ir,
            is_signed: self.is_signed,
        }
        .into())
    }

    fn extend_to(
        &self,
        target_type: &IntegerType,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        let value_type = self.value_type();
        let is_compatible = if value_type.is_signed == target_type.is_signed {
            value_type.width <= target_type.width
        } else if target_type.is_signed && !value_type.is_signed {
            value_type.width < target_type.width
        } else {
            false
        };

        if !is_compatible {
            return Err(CompilationError::TypeMismatch);
        }

        let builder = expr_translator.builder();
        let context = expr_translator.context();
        let result_ir = if target_type.is_signed {
            builder.build_int_s_extend(self.ir, target_type.to_ir(context), "")?
        } else {
            builder.build_int_z_extend(self.ir, target_type.to_ir(context), "")?
        };

        Ok(IntegerValue {
            ir: result_ir,
            is_signed: target_type.is_signed,
        })
    }
}
