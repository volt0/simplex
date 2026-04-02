use inkwell::context::Context;

use crate::errors::{CompilationError, CompilationResult};
use crate::types::Type;

type FloatTypeIR<'ctx> = inkwell::types::FloatType<'ctx>;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum FloatType {
    F32,
    F64,
}

impl Into<Type> for FloatType {
    fn into(self) -> Type {
        Type::Float(self)
    }
}

impl FloatType {
    pub fn combine_with(&self, other_type: &Type) -> CompilationResult<Self> {
        match other_type {
            Type::Float(other_type) => {
                if self > other_type {
                    Ok(self.clone())
                } else {
                    Ok(other_type.clone())
                }
            }
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    pub fn to_ir<'ctx>(&self, context: &'ctx Context) -> FloatTypeIR<'ctx> {
        match self {
            FloatType::F32 => context.f32_type(),
            FloatType::F64 => context.f64_type(),
        }
    }
}
