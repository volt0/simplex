use inkwell::context::Context;

type FloatTypeIR<'ctx> = inkwell::types::FloatType<'ctx>;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum FloatType {
    F32,
    F64,
}

impl FloatType {
    pub fn to_ir<'ctx>(&self, context: &'ctx Context) -> FloatTypeIR<'ctx> {
        match self {
            FloatType::F32 => context.f32_type(),
            FloatType::F64 => context.f64_type(),
        }
    }
}
