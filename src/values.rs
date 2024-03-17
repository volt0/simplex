use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::types::Type;

#[derive(Clone)]
pub struct Value<'ctx> {
    pub ir: AnyValueEnum<'ctx>,
    pub value_type: Type,
}

impl<'ctx> Value<'ctx> {
    pub fn compile_as_basic(&self) -> BasicValueEnum<'ctx> {
        self.ir.try_into().unwrap()
    }
}
