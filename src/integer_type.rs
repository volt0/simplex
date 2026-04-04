use inkwell::context::Context;
use inkwell::types::IntType;

use crate::errors::{CompilationError, CompilationResult};
use crate::types::Type;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerTypeWidth {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone)]
pub struct IntegerType<'ctx> {
    pub ir: IntType<'ctx>,
    pub is_signed: bool,
}

impl<'ctx> Into<IntegerTypeSpec> for IntegerType<'ctx> {
    fn into(self) -> IntegerTypeSpec {
        IntegerTypeSpec {
            is_signed: self.is_signed,
            width: self.width(),
        }
    }
}

impl<'ctx> IntegerType<'ctx> {
    pub fn new(type_spec: &IntegerTypeSpec, context: &'ctx Context) -> Self {
        IntegerType {
            ir: match type_spec.width {
                IntegerTypeWidth::I8 => context.i8_type(),
                IntegerTypeWidth::I16 => context.i16_type(),
                IntegerTypeWidth::I32 => context.i32_type(),
                IntegerTypeWidth::I64 => context.i64_type(),
            },
            is_signed: type_spec.is_signed,
        }
    }

    #[inline(always)]
    pub fn width(&self) -> IntegerTypeWidth {
        match self.ir.get_bit_width() {
            8 => IntegerTypeWidth::I8,
            16 => IntegerTypeWidth::I16,
            32 => IntegerTypeWidth::I32,
            64 => IntegerTypeWidth::I64,
            width => panic!("Invalid integer type width: {}", width),
        }
    }

    pub fn combine_with(&self, other_type: Type<'ctx>) -> CompilationResult<Self> {
        Ok(match other_type {
            Type::Integer(other_type) => {
                if self.is_signed == other_type.is_signed {
                    if other_type.width() > self.width() {
                        other_type.clone()
                    } else {
                        self.clone()
                    }
                } else if other_type.is_signed && other_type.width() > self.width() {
                    other_type.clone()
                } else if self.is_signed && self.width() > other_type.width() {
                    self.clone()
                } else {
                    return Err(CompilationError::TypeMismatch);
                }
            }
            Type::Bool => self.clone(),
            _ => return Err(CompilationError::TypeMismatch),
        })
    }
}

#[derive(Clone)]
pub struct IntegerTypeSpec {
    pub is_signed: bool,
    pub width: IntegerTypeWidth,
}
