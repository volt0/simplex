use crate::ast;
use crate::expression::{BinaryOperation, ExpressionCompiler};
use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValue, BasicValueEnum};

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

    pub fn compile_binary_operation<'ctx>(
        &self,
        op: BinaryOperation,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
        compiler: &ExpressionCompiler<'ctx, '_, '_, '_>,
    ) -> BasicValueEnum<'ctx> {
        let builder = compiler.builder();
        let lhs = lhs.into_int_value();
        let rhs = rhs.into_int_value();
        match op {
            BinaryOperation::Add => builder.build_int_add(lhs, rhs, "").unwrap(),
            BinaryOperation::Sub => todo!(),
            BinaryOperation::Mul => todo!(),
            BinaryOperation::Div => todo!(),
            BinaryOperation::Mod => todo!(),
            BinaryOperation::BitAnd => todo!(),
            BinaryOperation::BitXor => todo!(),
            BinaryOperation::BitOr => todo!(),
            BinaryOperation::ShiftLeft => todo!(),
            BinaryOperation::ShiftRight => todo!(),
            BinaryOperation::Eq => todo!(),
            BinaryOperation::Ne => todo!(),
            BinaryOperation::Gt => todo!(),
            BinaryOperation::Ge => todo!(),
            BinaryOperation::Lt => todo!(),
            BinaryOperation::Le => todo!(),
            BinaryOperation::LogicalAnd => todo!(),
            BinaryOperation::LogicalOr => todo!(),
        }
        .as_basic_value_enum()
    }
}

pub enum TypeHint {
    Explicit(TypeSpec),
    Inferred,
}

impl TypeHint {
    pub fn from_type_spec(type_spec: Option<&ast::TypeSpec>) -> Self {
        match type_spec {
            None => TypeHint::Inferred,
            Some(type_ast) => TypeHint::Explicit(TypeSpec::from_ast(type_ast)),
        }
    }
}
