use crate::ast;

mod float;
mod function;
mod integer;

pub use self::float::FloatType;
pub use self::function::FunctionType;
pub use self::integer::{IntegerType, IntegerTypeSize};

pub type TypeHint = Option<TypeSpec>;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSpec {
    Void,
    Bool,
    Integer(IntegerType),
    Float(FloatType),
}

impl TypeSpec {
    pub fn from_ast(type_spec_ast: &ast::Type) -> Self {
        match type_spec_ast {
            ast::Type::Integer(int_type) => TypeSpec::Integer(IntegerType::from_ast(int_type)),
            ast::Type::Identifier(_) => todo!(),
            ast::Type::Void => TypeSpec::Void,
            ast::Type::Boolean => TypeSpec::Bool,
            ast::Type::Float(float_type) => TypeSpec::Float(FloatType::from_ast(float_type)),
        }
    }
}
