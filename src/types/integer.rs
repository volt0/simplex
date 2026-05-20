use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;
use inkwell::IntPredicate;

use super::boolean::BoolValue;
use super::floating::{FloatType, FloatValue};
use super::primitive::{PrimitiveType, PrimitiveValue};
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};

type IntegerTypeIR<'ctx> = inkwell::types::IntType<'ctx>;
type IntegerValueIR<'ctx> = inkwell::values::IntValue<'ctx>;

#[derive(Clone, PartialEq)]
pub struct IntegerType<'ctx> {
    ir: IntegerTypeIR<'ctx>,
    is_signed: bool,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerWidth {
    I8,
    I16,
    I32,
    I64,
}

impl<'ctx> IntegerType<'ctx> {
    #[inline]
    pub fn from_ir(ir: IntegerTypeIR<'ctx>, is_signed: bool) -> Self {
        Self { ir, is_signed }
    }

    pub fn from_spec(context: &'ctx Context, width: IntegerWidth, is_signed: bool) -> Self {
        let ir = match width {
            IntegerWidth::I8 => context.i8_type(),
            IntegerWidth::I16 => context.i16_type(),
            IntegerWidth::I32 => context.i32_type(),
            IntegerWidth::I64 => context.i64_type(),
        };
        Self { ir, is_signed }
    }

    pub fn combined(lhs: IntegerType<'ctx>, rhs: IntegerType<'ctx>) -> CompilationResult<Self> {
        if lhs.is_signed == rhs.is_signed {
            if rhs.bit_width() > lhs.bit_width() {
                Ok(rhs)
            } else {
                Ok(lhs)
            }
        } else if rhs.is_signed && rhs.bit_width() > lhs.bit_width() {
            Ok(rhs)
        } else if lhs.is_signed && lhs.bit_width() > rhs.bit_width() {
            Ok(lhs)
        } else {
            Err(CompilationError::TypeMismatch)
        }
    }

    #[inline]
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }

    #[inline]
    pub fn ir(&self) -> &IntegerTypeIR<'ctx> {
        &self.ir
    }

    #[inline]
    fn bit_width(&self) -> u32 {
        self.ir.get_bit_width()
    }

    #[inline]
    pub fn new_i8(context: &'ctx Context, is_signed: bool) -> Self {
        Self::from_spec(context, IntegerWidth::I8, is_signed)
    }

    #[inline]
    pub fn new_i16(context: &'ctx Context, is_signed: bool) -> Self {
        Self::from_spec(context, IntegerWidth::I16, is_signed)
    }

    #[inline]
    pub fn new_i32(context: &'ctx Context, is_signed: bool) -> Self {
        Self::from_spec(context, IntegerWidth::I32, is_signed)
    }

    #[inline]
    pub fn new_i64(context: &'ctx Context, is_signed: bool) -> Self {
        Self::from_spec(context, IntegerWidth::I64, is_signed)
    }
}

impl<'ctx> Into<BasicTypeEnum<'ctx>> for IntegerType<'ctx> {
    fn into(self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::IntType(self.ir)
    }
}

impl<'ctx> From<IntegerType<'ctx>> for PrimitiveType<'ctx> {
    #[inline]
    fn from(value: IntegerType<'ctx>) -> Self {
        PrimitiveType::Integer(value)
    }
}

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    ir: IntegerValueIR<'ctx>,
    is_signed: bool,
}

impl<'ctx> IntegerValue<'ctx> {
    #[inline]
    pub fn from_ir(ir: IntegerValueIR<'ctx>, is_signed: bool) -> Self {
        IntegerValue { ir, is_signed }
    }

    pub fn from_constant(context: &'ctx Context, value: i32) -> Self {
        IntegerValue {
            ir: context.i32_type().const_int(value as u64, true),
            is_signed: true,
        }
    }

    pub fn get_type(&self) -> IntegerType<'ctx> {
        IntegerType::from_ir(self.ir.get_type(), self.is_signed)
    }

    pub fn to_bool(&self, builder: &Builder<'ctx>) -> CompilationResult<BoolValue<'ctx>> {
        let type_ir = self.ir.get_type();
        let result_ir = builder.build_int_compare(
            IntPredicate::NE,
            self.ir,
            type_ir.const_int(0, false),
            "",
        )?;
        Ok(BoolValue::from_ir(result_ir))
    }

    pub fn to_float(
        &self,
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

        Ok(FloatValue::from_ir(result_ir))
    }

    pub fn promote(
        &self,
        builder: &Builder<'ctx>,
        target_type: &IntegerType<'ctx>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        let this_type_ir = self.ir.get_type();
        let target_type_ir = target_type.ir.clone();
        let is_compatible = if self.is_signed == target_type.is_signed {
            this_type_ir.get_bit_width() <= target_type_ir.get_bit_width()
        } else if target_type.is_signed && !self.is_signed {
            this_type_ir.get_bit_width() < target_type_ir.get_bit_width()
        } else {
            false
        };

        if !is_compatible {
            return Err(CompilationError::TypeMismatch);
        }

        let result_ir = if target_type.is_signed {
            builder.build_int_s_extend(self.ir, target_type_ir, "")?
        } else {
            builder.build_int_z_extend(self.ir, target_type_ir, "")?
        };

        Ok(IntegerValue {
            ir: result_ir,
            is_signed: target_type.is_signed,
        })
    }

    pub fn do_binary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &IntegerValue<'ctx>,
    ) -> CompilationResult<()> {
        let result_type = IntegerType::combined(self.get_type(), other.get_type())?;
        let lhs_ir = self.clone().promote(builder, &result_type)?.ir;
        let rhs_ir = other.clone().promote(builder, &result_type)?.ir;

        self.ir = match op {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Sub => builder.build_int_sub(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Mul => builder.build_int_mul(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Div => {
                if self.is_signed {
                    builder.build_int_signed_div(lhs_ir, rhs_ir, "")?
                } else {
                    builder.build_int_unsigned_div(lhs_ir, rhs_ir, "")?
                }
            }
            BinaryOperation::Mod => {
                if self.is_signed {
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
                builder.build_right_shift(lhs_ir, rhs_ir, self.is_signed, "")?
            }
        };
        Ok(())
    }

    pub fn do_unary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<()> {
        self.ir = match op {
            UnaryOperation::Plus => self.ir,
            UnaryOperation::Minus => builder.build_int_neg(self.ir, "")?,
            UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
        };
        Ok(())
    }
}

impl<'ctx> Into<IntegerValueIR<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> IntegerValueIR<'ctx> {
        self.ir
    }
}

impl<'ctx> Into<PrimitiveValue<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> PrimitiveValue<'ctx> {
        PrimitiveValue::Integer(self)
    }
}
