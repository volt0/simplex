use crate::float_type::FloatType;
use crate::integer_type::IntegerType;

pub type TypeSpec = Type;

#[derive(Clone)]
pub enum Type {
    Integer(IntegerType),
    Float(FloatType),
    Bool,
}

impl<'ctx> Type {
    // pub fn combined_type(lhs_type: Type<'ctx>, rhs_type: Type<'ctx>) -> CompilationResult<Self> {
    //     let combined_type_spec = match lhs_type {
    //         Type::Integer(lhs_type) => Type::Integer(lhs_type.combine_with(rhs_type)?),
    //         Type::Float(lhs_type) => Type::Float(lhs_type.combine_with(rhs_type)?),
    //         Type::Bool => match rhs_type {
    //             Type::Bool => Type::Bool,
    //             _ => Self::combined_type(rhs_type, Type::Bool)?,
    //         },
    //     };
    //     Ok(combined_type_spec)
    // }
}
