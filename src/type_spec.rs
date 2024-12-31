use crate::ast;
use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};

#[derive(Clone)]
pub enum TypeSpec {
    I64,
}

impl TypeSpec {
    pub fn from_ast(type_spec_ast: &ast::TypeSpec) -> Self {
        match type_spec_ast {
            ast::TypeSpec::Integer(_) => TypeSpec::I64,
            ast::TypeSpec::Identifier(_) => todo!(),
            ast::TypeSpec::Void => todo!(),
            ast::TypeSpec::Boolean => todo!(),
            ast::TypeSpec::Float(_) => todo!(),
        }
    }

    pub fn into_ir(self, context: &Context) -> BasicTypeEnum {
        match self {
            TypeSpec::I64 => context.i64_type().as_basic_type_enum(),
        }
    }
}

pub enum TypeHint {
    Explicit(TypeSpec),
    Inferred,
}
