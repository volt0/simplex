use inkwell::context::Context;
use inkwell::types::IntType;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone)]
pub struct IntegerType {
    pub is_signed: bool,
    pub width: IntegerTypeSize,
}

impl IntegerType {
    pub fn to_ir<'ctx>(&self, context: &'ctx Context) -> IntType<'ctx> {
        match self.width {
            IntegerTypeSize::I8 => context.i8_type(),
            IntegerTypeSize::I16 => context.i16_type(),
            IntegerTypeSize::I32 => context.i32_type(),
            IntegerTypeSize::I64 => context.i64_type(),
        }
    }
}
