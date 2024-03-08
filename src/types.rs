#![allow(unused)]

use inkwell::context::Context as BackendContext;
use inkwell::types::{BasicTypeEnum, FloatType as FloatTypeIR, IntType as IntTypeIR};

#[derive(Clone)]
pub enum Type {
    Void,
    Boolean,
    SignedInteger(IntegerType),
    UnsignedInteger(IntegerType),
    Float(FloatType),
}

impl Type {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> BasicTypeEnum<'ctx> {
        match self {
            Type::Void => BasicTypeEnum::StructType(ctx.struct_type(&vec![], true)),
            Type::Boolean => BasicTypeEnum::IntType(ctx.bool_type()),
            Type::SignedInteger(int_type) | Type::UnsignedInteger(int_type) => {
                BasicTypeEnum::IntType(int_type.compile(ctx))
            }
            Type::Float(float_type) => BasicTypeEnum::FloatType(float_type.compile(ctx)),
        }
    }
}

#[derive(Clone)]
pub enum IntegerType {
    Byte,
    Short,
    Int,
    Long,
}

impl IntegerType {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> IntTypeIR<'ctx> {
        match self {
            IntegerType::Byte => ctx.i8_type(),
            IntegerType::Short => ctx.i16_type(),
            IntegerType::Int => ctx.i32_type(),
            IntegerType::Long => ctx.i64_type(),
        }
    }
}

#[derive(Clone)]
pub enum FloatType {
    Float,
    Double,
}

impl FloatType {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> FloatTypeIR<'ctx> {
        match self {
            FloatType::Float => ctx.f32_type(),
            FloatType::Double => ctx.f64_type(),
        }
    }
}
