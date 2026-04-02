use crate::errors::CompilationResult;
use crate::float_type::FloatType;
use crate::integer_type::IntegerType;

#[derive(Clone)]
pub enum Type {
    Integer(IntegerType),
    Float(FloatType),
    Bool,
}

impl Type {
    pub fn combined_type(lhs_type: &Type, rhs_type: &Type) -> CompilationResult<Self> {
        match lhs_type {
            Type::Integer(lhs_type) => Ok(lhs_type.combine_with(rhs_type)?.into()),
            Type::Float(lhs_type) => Ok(lhs_type.combine_with(rhs_type)?.into()),
            Type::Bool => match rhs_type {
                Type::Bool => Ok(Type::Bool),
                _ => Self::combined_type(rhs_type, &Type::Bool),
            },
        }
    }
}
