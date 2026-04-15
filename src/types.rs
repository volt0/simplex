use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use crate::ast::IntegerTypeWidth;
use crate::errors::CompilationError;
use crate::float_type::{FloatType, FloatTypeWidth};
use crate::function_type::FunctionType;
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

pub type BoolTypeIR<'ctx> = inkwell::types::IntType<'ctx>;

#[derive(Clone, PartialEq)]
pub enum Type<'ctx> {
    Integer(IntegerType<'ctx>),
    Float(FloatType<'ctx>),
    Bool(BoolTypeIR<'ctx>),
    Function(FunctionType<'ctx>),
}

impl<'ctx> Type<'ctx> {
    pub fn from_spec(context: &'ctx Context, type_spec: TypeSpec) -> Self {
        match type_spec {
            TypeSpec::Integer { width, is_signed } => {
                Type::Integer(IntegerType::from_spec(context, width, is_signed))
            }
            TypeSpec::Float { width } => Type::Float(FloatType::from_spec(context, width)),
            TypeSpec::Bool => Type::Bool(context.bool_type()),
        }
    }
}

impl<'ctx> TryInto<BasicTypeEnum<'ctx>> for Type<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicTypeEnum<'ctx>, Self::Error> {
        Ok(match self {
            Type::Integer(int_type) => BasicTypeEnum::IntType(int_type.into()),
            Type::Float(float_type) => BasicTypeEnum::FloatType(float_type.into()),
            Type::Bool(ir) => BasicTypeEnum::IntType(ir),
            _ => return Err(CompilationError::InvalidOperation),
        })
    }
}
