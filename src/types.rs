use inkwell::context::Context as BackendContext;
use inkwell::types::{BasicTypeEnum, FloatType as FloatTypeIR, IntType as IntTypeIR};

#[allow(unused)]
#[derive(Clone, PartialEq)]
pub enum TypeSpec {
    Void,
    Boolean,
    SignedInteger(IntegerType),
    UnsignedInteger(IntegerType),
    Float(FloatType),
}

impl TypeSpec {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> BasicTypeEnum<'ctx> {
        match self {
            TypeSpec::Void => BasicTypeEnum::StructType(ctx.struct_type(&vec![], true)),
            TypeSpec::Boolean => BasicTypeEnum::IntType(ctx.bool_type()),
            TypeSpec::SignedInteger(int_type) | TypeSpec::UnsignedInteger(int_type) => {
                BasicTypeEnum::IntType(int_type.compile(ctx))
            }
            TypeSpec::Float(float_type) => BasicTypeEnum::FloatType(float_type.compile(ctx)),
        }
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq)]
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

#[allow(unused)]
#[derive(Clone, PartialEq)]
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
