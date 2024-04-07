use inkwell::context::Context as BackendContext;
use inkwell::types::{BasicTypeEnum, FloatType as FloatTypeIR, IntType as IntTypeIR};

#[allow(unused)]
#[derive(Clone, PartialEq)]
pub enum TypeSpec {
    Void,
    Boolean,
    Integer(IntegerTypeSpec),
    Float(FloatType),
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
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> IntTypeIR<'ctx> {
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

impl TypeSpec {
    #[inline(always)]
    pub fn new_byte() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Byte,
            sign_extend: true,
        })
    }

    #[inline(always)]
    pub fn new_short() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Short,
            sign_extend: true,
        })
    }

    #[inline(always)]
    pub fn new_int() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Int,
            sign_extend: true,
        })
    }

    #[inline(always)]
    pub fn new_long() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Long,
            sign_extend: true,
        })
    }

    #[inline(always)]
    pub fn new_ubyte() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Byte,
            sign_extend: false,
        })
    }

    #[inline(always)]
    pub fn new_ushort() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Short,
            sign_extend: false,
        })
    }

    #[inline(always)]
    pub fn new_uint() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Int,
            sign_extend: false,
        })
    }

    #[inline(always)]
    pub fn new_ulong() -> Self {
        TypeSpec::Integer(IntegerTypeSpec {
            size: IntegerSize::Long,
            sign_extend: false,
        })
    }
}
