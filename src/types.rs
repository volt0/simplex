use inkwell::context::Context as BackendContext;
use inkwell::types::{BasicTypeEnum, FloatType, IntType, VoidType};

use crate::ast;

#[derive(Clone)]
pub enum Type<'ctx> {
    Void(VoidType<'ctx>),
    Boolean(IntType<'ctx>),
    Integer(IntegerType<'ctx>),
    Float(FloatType<'ctx>),
}

#[derive(Clone)]
pub struct IntegerType<'ctx> {
    ir: IntType<'ctx>,
    sign_extend: bool,
}

impl<'ctx> TryInto<BasicTypeEnum<'ctx>> for Type<'ctx> {
    type Error = ();

    fn try_into(self) -> Result<BasicTypeEnum<'ctx>, Self::Error> {
        match self {
            Type::Void(_) => Err(()),
            Type::Boolean(inner) => Ok(BasicTypeEnum::IntType(inner)),
            Type::Integer(inner) => Ok(BasicTypeEnum::IntType(inner.ir)),
            Type::Float(inner) => Ok(BasicTypeEnum::FloatType(inner)),
        }
    }
}

impl<'ctx> Type<'ctx> {
    pub fn compile(type_ast: ast::TypeSpec, ctx: &'ctx BackendContext) -> Self {
        match type_ast {
            ast::TypeSpec::Identifier(_) => todo!(),
            ast::TypeSpec::Void => Type::Void(ctx.void_type()),
            ast::TypeSpec::Boolean => Type::Boolean(ctx.bool_type()),
            ast::TypeSpec::Integer(int_type) => Type::Integer(match int_type {
                ast::IntegerType::I8 => IntegerType {
                    ir: ctx.i8_type(),
                    sign_extend: true,
                },
                ast::IntegerType::I16 => IntegerType {
                    ir: ctx.i16_type(),
                    sign_extend: true,
                },
                ast::IntegerType::I32 => IntegerType {
                    ir: ctx.i32_type(),
                    sign_extend: true,
                },
                ast::IntegerType::I64 => IntegerType {
                    ir: ctx.i64_type(),
                    sign_extend: true,
                },
                ast::IntegerType::U8 => IntegerType {
                    ir: ctx.i8_type(),
                    sign_extend: false,
                },
                ast::IntegerType::U16 => IntegerType {
                    ir: ctx.i16_type(),
                    sign_extend: false,
                },
                ast::IntegerType::U32 => IntegerType {
                    ir: ctx.i32_type(),
                    sign_extend: false,
                },
                ast::IntegerType::U64 => IntegerType {
                    ir: ctx.i64_type(),
                    sign_extend: false,
                },
            }),
            ast::TypeSpec::Float(float_type) => Type::Float(match float_type {
                ast::FloatType::F32 => ctx.f32_type(),
                ast::FloatType::F64 => ctx.f64_type(),
            }),
        }
    }
}
