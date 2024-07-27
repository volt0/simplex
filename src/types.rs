use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::types::{BasicTypeEnum, FloatType, IntType};

pub enum TypeHint<'ctx> {
    InferencePending,
    Known(Type<'ctx>),
}

pub enum Type<'ctx> {
    Integer(IntegerType<'ctx>),
}

#[derive(Clone)]
pub struct IntegerType<'ctx> {
    pub ir: IntType<'ctx>,
    pub sign_extend: bool,
}


#[allow(unused)]
#[derive(Clone, PartialEq)]
pub enum TypeSpec {
    Void,
    Boolean,
    Integer(IntegerTypeSpec),
    Float(FloatTypeSpec),
}

impl TypeSpec {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> BasicTypeEnum<'ctx> {
        match self {
            TypeSpec::Void => BasicTypeEnum::StructType(ctx.struct_type(&vec![], true)),
            TypeSpec::Boolean => BasicTypeEnum::IntType(ctx.bool_type()),
            TypeSpec::Integer(int_type) => BasicTypeEnum::IntType(int_type.compile(ctx)),
            TypeSpec::Float(float_type) => BasicTypeEnum::FloatType(float_type.compile(ctx)),
        }
    }
}

impl TypeSpec {
    #[inline(always)]
    pub fn new_integer(size: IntegerSize) -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size,
            sign_extend: true,
        })
    }

    #[inline(always)]
    pub fn new_unsigned_integer(size: IntegerSize) -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size,
            sign_extend: false,
        })
    }

    #[inline(always)]
    pub fn new_identifier(name: Rc<str>) -> Self {
        _ = name;
        todo!()
    }
}

#[derive(Clone, PartialEq)]
pub struct IntegerTypeSpec {
    pub size: IntegerSize,
    pub sign_extend: bool,
}

#[allow(unused)]
#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerSize {
    Byte,
    Short,
    Int,
    Long,
}

impl IntegerTypeSpec {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> IntType<'ctx> {
        match self.size {
            IntegerSize::Byte => ctx.i8_type(),
            IntegerSize::Short => ctx.i16_type(),
            IntegerSize::Int => ctx.i32_type(),
            IntegerSize::Long => ctx.i64_type(),
        }
    }
}

#[allow(unused)]
#[derive(Clone, PartialEq)]
pub enum FloatTypeSpec {
    Float,
    Double,
}

impl FloatTypeSpec {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> FloatType<'ctx> {
        match self {
            FloatTypeSpec::Float => ctx.f32_type(),
            FloatTypeSpec::Double => ctx.f64_type(),
        }
    }
}
