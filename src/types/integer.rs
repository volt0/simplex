use crate::ast;

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
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
