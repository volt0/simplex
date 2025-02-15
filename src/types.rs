use crate::ast;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    I64,
}

impl Type {
    pub fn from_ast(type_spec_ast: &ast::TypeSpec) -> Self {
        match type_spec_ast {
            ast::TypeSpec::Integer(_) => Type::I64,
            ast::TypeSpec::Identifier(_) => todo!(),
            ast::TypeSpec::Void => todo!(),
            ast::TypeSpec::Boolean => todo!(),
            ast::TypeSpec::Float(_) => todo!(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum TypeHint {
    Explicit(Type),
    Inferred,
}

impl TypeHint {
    pub fn from_type_spec(type_spec: Option<&ast::TypeSpec>) -> Self {
        match type_spec {
            None => TypeHint::Inferred,
            Some(type_ast) => TypeHint::Explicit(Type::from_ast(type_ast)),
        }
    }
}
