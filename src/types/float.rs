use crate::ast;

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
