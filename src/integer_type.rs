use crate::errors::{CompilationError, CompilationResult};
use crate::types::Type;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone)]
pub struct IntegerType {
    pub is_signed: bool,
    pub width: IntegerTypeSize,
}

impl Into<Type> for IntegerType {
    fn into(self) -> Type {
        Type::Integer(self)
    }
}

impl IntegerType {
    pub fn combine_with(&self, other_type: &Type) -> CompilationResult<Self> {
        Ok(match other_type {
            Type::Integer(other_type) => {
                if self.is_signed == other_type.is_signed {
                    if other_type.width > self.width {
                        other_type.clone()
                    } else {
                        self.clone()
                    }
                } else if other_type.is_signed && other_type.width > self.width {
                    other_type.clone()
                } else if self.is_signed && self.width > other_type.width {
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
