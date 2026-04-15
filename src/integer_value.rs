use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::IntValue;
use inkwell::IntPredicate;

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::float_type::FloatType;
use crate::float_value::FloatValue;
use crate::integer_type::IntegerType;
use crate::value::Value;

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    ir: IntValue<'ctx>,
    is_signed: bool,
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
    pub fn new(ir: IntValue<'ctx>, is_signed: bool) -> Self {
        IntegerValue { ir, is_signed }
    }

    pub fn from_constant(context: &'ctx Context, value: i32) -> Self {
        IntegerValue {
            ir: context.i32_type().const_int(value as u64, true),
            is_signed: true,
        }
    }

    pub fn get_type(&self) -> IntegerType<'ctx> {
        IntegerType::new(self.ir.get_type(), self.is_signed)
    }

    pub fn to_bool(self, builder: &Builder<'ctx>) -> CompilationResult<BoolValue<'ctx>> {
        let type_ir = self.ir.get_type();
        let result_ir = builder.build_int_compare(
            IntPredicate::NE,
            self.ir,
            type_ir.const_int(0, false),
            "",
        )?;
        Ok(BoolValue::new(result_ir))
    }

    pub fn to_float(
        self,
        builder: &Builder<'ctx>,
        required_type: &FloatType<'ctx>,
    ) -> CompilationResult<FloatValue<'ctx>> {
        let value_bit_width = self.ir.get_type().get_bit_width();
        if match required_type.bit_width() {
            32 => value_bit_width > 23,
            64 => value_bit_width > 52,
            _ => unimplemented!(),
        } {
            return Err(CompilationError::TypeMismatch);
        }

        let result_type_ir = required_type.ir();
        let result_ir = if self.is_signed {
            builder.build_signed_int_to_float(self.ir, result_type_ir.clone(), "")?
        } else {
            builder.build_unsigned_int_to_float(self.ir, result_type_ir.clone(), "")?
        };

        Ok(FloatValue::new(result_ir))
    }

    pub fn binary_operation(
        self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let other = match other {
            Value::Integer(other) => other,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let lhs_ir = self.ir;
        let rhs_ir = other.ir;
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

        Ok(Self {
            ir: result_ir?,
            is_signed: self.is_signed,
        }
        .into())
    }

    pub fn unary_operation(
        self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<Value<'ctx>> {
        let result_ir = match op {
            UnaryOperation::Plus => self.ir,
            UnaryOperation::Minus => builder.build_int_neg(self.ir, "")?,
            UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
        };

        Ok(Self {
            ir: result_ir,
            is_signed: self.is_signed,
        }
        .into())
    }

    pub fn extend(
        self,
        builder: &Builder<'ctx>,
        target_type: &IntegerType<'ctx>,
    ) -> CompilationResult<Self> {
        if !self.get_type().is_compatible(target_type) {
            return Err(CompilationError::TypeMismatch);
        }

        let target_type_ir = target_type.ir();
        let result_ir = if target_type.is_signed() {
            builder.build_int_s_extend(self.ir, target_type_ir.clone(), "")?
        } else {
            builder.build_int_z_extend(self.ir, target_type_ir.clone(), "")?
        };

        Ok(IntegerValue {
            ir: result_ir,
            is_signed: target_type.is_signed(),
        })
    }
}
