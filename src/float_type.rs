use inkwell::context::Context;

use crate::errors::{CompilationError, CompilationResult};
use crate::types::Type;

type FloatTypeIR<'ctx> = inkwell::types::FloatType<'ctx>;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum FloatTypeWidth {
    F32,
    F64,
}

#[repr(transparent)]
#[derive(Clone)]
pub struct FloatType<'ctx> {
    pub ir: FloatTypeIR<'ctx>,
}

impl<'ctx> Into<FloatTypeSpec> for FloatType<'ctx> {
    fn into(self) -> FloatTypeSpec {
        match self.ir.get_bit_width() {
            32 => FloatTypeSpec::F32,
            64 => FloatTypeSpec::F64,
            width => panic!("Invalid float type width: {}", width),
        }
    }
}

impl<'ctx> FloatType<'ctx> {
    pub fn new(type_spec: &FloatTypeSpec, context: &'ctx Context) -> Self {
        FloatType {
            ir: match type_spec {
                FloatTypeSpec::F32 => context.f32_type(),
                FloatTypeSpec::F64 => context.f64_type(),
            },
        }
    }

    pub fn width(&self) -> FloatTypeWidth {
        match self.ir.get_bit_width() {
            32 => FloatTypeWidth::F32,
            64 => FloatTypeWidth::F64,
            width => panic!("Invalid float type width: {}", width),
        }
    }

    pub fn combine_with(&self, other_type: Type<'ctx>) -> CompilationResult<Self> {
        match other_type {
            Type::Float(other_type) => {
                if self.width() > other_type.width() {
                    Ok(self.clone())
                } else {
                    Ok(other_type.clone())
                }
            }
            _ => Err(CompilationError::TypeMismatch),
        }
    }
}

pub type FloatTypeSpec = FloatTypeWidth;
