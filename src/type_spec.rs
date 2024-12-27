use inkwell::context::Context;
use inkwell::types::{BasicType, BasicTypeEnum};

#[derive(Clone)]
pub enum TypeSpec {
    I64,
}

impl TypeSpec {
    pub fn into_ir(self, context: &Context) -> BasicTypeEnum {
        match self {
            TypeSpec::I64 => context.i64_type().as_basic_type_enum(),
        }
    }
}
