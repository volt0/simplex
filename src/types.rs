use crate::ast;
use crate::expression::{BinaryOperation, ExpressionCompiler};
use crate::function::Function;
use crate::module::ModuleCompiler;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicValue, BasicValueEnum};
use std::ops::Deref;

#[derive(Clone, Debug)]
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

#[repr(transparent)]
pub struct TypeCompiler<'ctx, 'm> {
    parent: &'m ModuleCompiler<'ctx>,
}

impl<'ctx, 'm> Deref for TypeCompiler<'ctx, 'm> {
    type Target = ModuleCompiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> TypeCompiler<'ctx, 'm> {
    pub fn new(parent: &'m ModuleCompiler<'ctx>) -> Self {
        TypeCompiler { parent }
    }

    pub fn compile_function_type(&self, function: &Function) -> FunctionType<'ctx> {
        let return_type = function.return_type();
        let return_type_ir = self.compile_type(&return_type);

        let arg_type_irs: Vec<BasicMetadataTypeEnum> = function
            .iter_args()
            .map(|arg| {
                let arg_type = arg.arg_type();
                self.compile_type(&arg_type).into()
            })
            .collect();

        return_type_ir.fn_type(&arg_type_irs, false)
    }

    pub fn compile_type(&self, type_spec: &Type) -> BasicTypeEnum<'ctx> {
        let context = self.context();
        match type_spec {
            Type::I64 => context.i64_type().as_basic_type_enum(),
        }
    }
}
