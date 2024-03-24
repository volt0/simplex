use std::ops::Deref;

use inkwell::values::{BasicValueEnum, IntValue};

#[derive(Clone)]
pub struct Value<'ctx> {
    pub ir: BasicValueEnum<'ctx>,
    pub sign_extend: bool,
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(ir: BasicValueEnum<'ctx>) -> Self {
        Value {
            ir,
            sign_extend: false,
        }
    }

    pub fn new_integer(ir: IntValue<'ctx>, sign_extend: bool) -> Self {
        Value {
            ir: ir.into(),
            sign_extend,
        }
    }
}

impl<'ctx> Deref for Value<'ctx> {
    type Target = BasicValueEnum<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.ir
    }
}
