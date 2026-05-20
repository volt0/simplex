use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use super::primitive::{PrimitiveType, PrimitiveValue};
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};

type FloatTypeIR<'ctx> = inkwell::types::FloatType<'ctx>;
type FloatValueIR<'ctx> = inkwell::values::FloatValue<'ctx>;

#[derive(Clone, PartialEq)]
pub struct FloatType<'ctx> {
    ir: FloatTypeIR<'ctx>,
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum FloatWidth {
    F32,
    F64,
}

impl<'ctx> FloatType<'ctx> {
    #[inline]
    pub fn from_ir(ir: FloatTypeIR<'ctx>) -> Self {
        Self { ir }
    }

    pub fn from_spec(context: &'ctx Context, width: FloatWidth) -> Self {
        Self {
            ir: match width {
                FloatWidth::F32 => context.f32_type(),
                FloatWidth::F64 => context.f64_type(),
            },
        }
    }

    #[inline]
    fn combined(lhs: FloatType<'ctx>, rhs: FloatType<'ctx>) -> CompilationResult<Self> {
        if rhs.bit_width() > lhs.bit_width() {
            Ok(rhs)
        } else {
            Ok(lhs)
        }
    }

    #[inline]
    pub fn ir(&self) -> &FloatTypeIR<'ctx> {
        &self.ir
    }

    #[inline]
    pub fn bit_width(&self) -> u32 {
        self.ir.get_bit_width()
    }

    #[inline]
    pub fn new_f32(context: &'ctx Context) -> Self {
        Self::from_spec(context, FloatWidth::F32)
    }

    #[inline]
    pub fn new_f64(context: &'ctx Context) -> Self {
        Self::from_spec(context, FloatWidth::F64)
    }
}

impl<'ctx> Into<BasicTypeEnum<'ctx>> for FloatType<'ctx> {
    fn into(self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::FloatType(self.ir)
    }
}

impl<'ctx> From<FloatType<'ctx>> for PrimitiveType<'ctx> {
    #[inline]
    fn from(value: FloatType<'ctx>) -> Self {
        PrimitiveType::Float(value)
    }
}

#[derive(Clone)]
#[repr(transparent)]
pub struct FloatValue<'ctx> {
    ir: FloatValueIR<'ctx>,
}

impl<'ctx> FloatValue<'ctx> {
    #[inline]
    pub fn from_ir(ir: FloatValueIR<'ctx>) -> Self {
        FloatValue { ir }
    }

    #[inline]
    pub fn get_type(&self) -> FloatType<'ctx> {
        FloatType::from_ir(self.ir.get_type())
    }

    pub fn promote(
        &self,
        builder: &Builder<'ctx>,
        target_type: &FloatType<'ctx>,
    ) -> CompilationResult<Self> {
        let self_type_ir = self.ir.get_type();
        if self_type_ir.get_bit_width() > target_type.bit_width() {
            return Err(CompilationError::TypeMismatch);
        }

        let result_type_ir = target_type.ir().clone();
        let result_ir = builder.build_float_ext(self.ir, result_type_ir, "")?;
        Ok(Self { ir: result_ir })
    }

    pub fn do_binary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &FloatValue<'ctx>,
    ) -> CompilationResult<()> {
        let result_type = FloatType::combined(self.get_type(), other.get_type())?;
        let lhs_ir = self.clone().promote(builder, &result_type)?.ir;
        let rhs_ir = other.clone().promote(builder, &result_type)?.ir;
        self.ir = match op {
            BinaryOperation::Add => builder.build_float_add(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Sub => builder.build_float_sub(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Mul => builder.build_float_mul(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Div => builder.build_float_div(lhs_ir, rhs_ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(())
    }

    pub fn do_unary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<()> {
        self.ir = match op {
            UnaryOperation::Plus => self.ir.clone(),
            UnaryOperation::Minus => builder.build_float_neg(self.ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(())
    }
}

impl<'ctx> Into<FloatValueIR<'ctx>> for FloatValue<'ctx> {
    fn into(self) -> FloatValueIR<'ctx> {
        self.ir
    }
}

impl<'ctx> Into<PrimitiveValue<'ctx>> for FloatValue<'ctx> {
    fn into(self) -> PrimitiveValue<'ctx> {
        PrimitiveValue::Float(self)
    }
}
