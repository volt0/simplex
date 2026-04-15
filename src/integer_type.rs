use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::errors::{CompilationError, CompilationResult};
use crate::integer_value::IntegerValue;
use crate::value::Value;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerTypeWidth {
    I8,
    I16,
    I32,
    I64,
}

pub type IntegerTypeIR<'ctx> = inkwell::types::IntType<'ctx>;

#[derive(Clone, PartialEq)]
pub struct IntegerType<'ctx> {
    ir: IntegerTypeIR<'ctx>,
    is_signed: bool,
}

impl<'ctx> Into<IntegerTypeIR<'ctx>> for IntegerType<'ctx> {
    fn into(self) -> IntegerTypeIR<'ctx> {
        self.ir
    }
}

impl<'ctx> IntegerType<'ctx> {
    #[inline]
    pub fn new(ir: IntegerTypeIR<'ctx>, is_signed: bool) -> Self {
        Self { ir, is_signed }
    }

    pub fn from_spec(context: &'ctx Context, width: IntegerTypeWidth, is_signed: bool) -> Self {
        let ir = match width {
            IntegerTypeWidth::I8 => context.i8_type(),
            IntegerTypeWidth::I16 => context.i16_type(),
            IntegerTypeWidth::I32 => context.i32_type(),
            IntegerTypeWidth::I64 => context.i64_type(),
        };
        Self { ir, is_signed }
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
    pub fn bit_width(&self) -> u32 {
        self.ir.get_bit_width()
    }

    pub fn is_compatible(&self, other_type: &IntegerType<'ctx>) -> bool {
        if self.is_signed == other_type.is_signed {
            self.bit_width() <= other_type.bit_width()
        } else if other_type.is_signed && !self.is_signed {
            self.bit_width() < other_type.bit_width()
        } else {
            false
        }
    }

    pub fn validate_value(
        &self,
        builder: &Builder<'ctx>,
        value: Value<'ctx>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        match value {
            Value::Integer(value) => value.extend(builder, self),
            Value::Bool(value) => value.to_integer(builder, self),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn combine_with(self, other: Self) -> CompilationResult<Self> {
        if self.is_signed == other.is_signed {
            if other.bit_width() > self.bit_width() {
                Ok(other)
            } else {
                Ok(self)
            }
        } else if other.is_signed && other.bit_width() > self.bit_width() {
            Ok(other)
        } else if self.is_signed && self.bit_width() > other.bit_width() {
            Ok(self)
        } else {
            Err(CompilationError::TypeMismatch)
        }
    }
}
