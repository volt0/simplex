use inkwell::context::Context;

use crate::errors::CompilationResult;
use crate::float_type::{FloatType, FloatTypeSpec};
use crate::integer_type::{IntegerType, IntegerTypeSpec};

#[derive(Clone)]
pub enum TypeSpec {
    Integer(IntegerTypeSpec),
    Float(FloatTypeSpec),
    Bool,
}

#[derive(Clone)]
pub enum Type<'ctx> {
    Integer(IntegerType<'ctx>),
    Float(FloatType<'ctx>),
    Bool,
}

impl<'ctx> Type<'ctx> {
    pub fn new(type_spec: &TypeSpec, context: &'ctx Context) -> Self {
        match type_spec {
            TypeSpec::Integer(type_spec) => Type::Integer(IntegerType::new(type_spec, context)),
            TypeSpec::Float(type_spec) => Type::Float(FloatType::new(type_spec, context)),
            TypeSpec::Bool => Type::Bool,
        }
    }

    pub fn combined_type(lhs_type: Type<'ctx>, rhs_type: Type<'ctx>) -> CompilationResult<Self> {
        let combined_type_spec = match lhs_type {
            Type::Integer(lhs_type) => Type::Integer(lhs_type.combine_with(rhs_type)?),
            Type::Float(lhs_type) => Type::Float(lhs_type.combine_with(rhs_type)?),
            Type::Bool => match rhs_type {
                Type::Bool => Type::Bool,
                _ => Self::combined_type(rhs_type, Type::Bool)?,
            },
        };
        Ok(combined_type_spec)
    }
}
