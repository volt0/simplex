use inkwell::context::Context;

use crate::ast::IntegerTypeWidth;
use crate::float_type::{FloatType, FloatTypeWidth};
use crate::integer_type::IntegerType;

#[derive(Clone)]
pub enum TypeSpec {
    Integer {
        width: IntegerTypeWidth,
        is_signed: bool,
    },
    Float {
        width: FloatTypeWidth,
    },
    Bool,
}

#[derive(Clone)]
pub enum Type<'ctx> {
    Integer(IntegerType<'ctx>),
    Float(FloatType<'ctx>),
    Bool,
}

impl<'ctx> Type<'ctx> {
    pub fn new(context: &'ctx Context, type_spec: TypeSpec) -> Self {
        match type_spec {
            TypeSpec::Integer { width, is_signed } => {
                Type::Integer(IntegerType::from_spec(context, width, is_signed))
            }
            TypeSpec::Float { width } => Type::Float(FloatType::from_spec(context, width)),
            TypeSpec::Bool => Type::Bool,
        }
    }

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
