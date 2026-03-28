use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::errors::CompilationError;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::value::Value;

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub(crate) ir: IntValue<'ctx>,
    pub value_type: IntegerType,
}

impl<'ctx> Into<Value<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Integer(self)
    }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn from_constant(value: i32, context: &'ctx Context) -> Self {
        IntegerValue {
            ir: context.i32_type().const_int(value as u64, true),
            value_type: IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I32,
            },
        }
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &IntegerValue<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Result<Self, CompilationError> {
        let lhs_type = self.value_type.clone();
        let rhs_type = other.value_type.clone();
        let result_type = if lhs_type.is_signed == rhs_type.is_signed {
            if rhs_type.width > lhs_type.width {
                rhs_type
            } else {
                lhs_type
            }
        } else if rhs_type.is_signed && rhs_type.width > lhs_type.width {
            rhs_type
        } else if lhs_type.is_signed && lhs_type.width > rhs_type.width {
            lhs_type
        } else {
            return Err(CompilationError::TypeMismatch);
        };

        let lhs_ir = self.to_ir_expanded(&result_type, builder, context)?;
        let rhs_ir = other.to_ir_expanded(&result_type, builder, context)?;
        let result_ir = match operation {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Sub => builder.build_int_sub(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Mul => builder.build_int_mul(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Div => {
                if self.value_type.is_signed {
                    builder.build_int_signed_div(lhs_ir, rhs_ir, "")?
                } else {
                    builder.build_int_unsigned_div(lhs_ir, rhs_ir, "")?
                }
            }
            BinaryOperation::Mod => {
                if self.value_type.is_signed {
                    builder.build_int_signed_rem(lhs_ir, rhs_ir, "")?
                } else {
                    builder.build_int_unsigned_rem(lhs_ir, rhs_ir, "")?
                }
            }
            BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, "")?,
            BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, "")?,
            BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, "")?,
            BinaryOperation::ShiftLeft => builder.build_left_shift(lhs_ir, rhs_ir, "")?,
            BinaryOperation::ShiftRight => {
                builder.build_right_shift(lhs_ir, rhs_ir, self.value_type.is_signed, "")?
            }
        };

        Ok(IntegerValue {
            ir: result_ir,
            value_type: result_type,
        })
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Result<Self, CompilationError> {
        let arg_type = self.value_type.clone();
        let arg_ir = self.to_ir_expanded(&arg_type, builder, context)?;
        let result_ir = match operation {
            UnaryOperation::Plus => self.ir.clone(),
            UnaryOperation::Minus => builder.build_int_neg(arg_ir, "")?,
            UnaryOperation::BitNot => builder.build_not(arg_ir, "")?,
        };
        Ok(IntegerValue {
            ir: result_ir,
            value_type: arg_type,
        })
    }

    fn to_ir_expanded(
        &self,
        new_type: &IntegerType,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Result<IntValue<'ctx>, CompilationError> {
        Ok(if new_type.is_signed {
            builder.build_int_s_extend(self.ir, new_type.to_ir(context), "")?
        } else {
            builder.build_int_z_extend(self.ir, new_type.to_ir(context), "")?
        })
    }
}
