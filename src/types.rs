use crate::ast;
use crate::function::FunctionSignature;
use std::rc::Rc;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Primitive(PrimitiveType),
    Function(Rc<FunctionSignature>),
}

impl Type {
    pub fn from_ast(type_spec_ast: &ast::Type) -> Self {
        match type_spec_ast {
            ast::Type::Integer(int_type) => {
                Type::Primitive(PrimitiveType::Integer(IntegerType::from_ast(int_type)))
            }
            ast::Type::Identifier(_) => todo!(),
            ast::Type::Void => Type::Primitive(PrimitiveType::Void),
            ast::Type::Boolean => Type::Primitive(PrimitiveType::Bool),
            ast::Type::Float(float_type) => {
                Type::Primitive(PrimitiveType::Float(FloatType::from_ast(float_type)))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveType {
    Void,
    Bool,
    Integer(IntegerType),
    Float(FloatType),
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntegerType {
    pub is_signed: bool,
    pub width: IntegerTypeSize,
}

impl IntegerType {
    pub fn from_ast(int_type_ast: &ast::IntegerType) -> Self {
        match int_type_ast {
            ast::IntegerType::I8 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I8,
            },
            ast::IntegerType::I16 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I16,
            },
            ast::IntegerType::I32 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I32,
            },
            ast::IntegerType::I64 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I64,
            },
            ast::IntegerType::U8 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I8,
            },
            ast::IntegerType::U16 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I16,
            },
            ast::IntegerType::U32 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I32,
            },
            ast::IntegerType::U64 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I64,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum FloatType {
    F32,
}

impl FloatType {
    pub fn from_ast(float_type_ast: &ast::FloatType) -> Self {
        match float_type_ast {
            ast::FloatType::F32 => FloatType::F32,
            ast::FloatType::F64 => FloatType::F32,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeHint {
    Explicit(Type),
    Inferred,
}

impl TypeHint {
    pub fn from_ast(type_spec_ast: Option<&ast::Type>) -> Self {
        match type_spec_ast {
            None => TypeHint::Inferred,
            Some(type_ast) => TypeHint::Explicit(Type::from_ast(type_ast)),
        }
    }
}
