use std::ops::Deref;

use inkwell::values::{BasicValueEnum, IntValue};

// #[derive(Clone)]
// pub enum Value<'ctx> {
//     SignedInt(IntValue<'ctx>),
//     UnsignedInt(IntValue<'ctx>),
// }

#[derive(Clone)]
pub struct Value<'ctx> {
    pub ir: BasicValueEnum<'ctx>,
    pub is_unsigned: bool,
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(ir: BasicValueEnum<'ctx>) -> Self {
        Value {
            ir,
            is_unsigned: false,
        }
    }

    pub fn new_integer(ir: IntValue<'ctx>, is_unsigned: bool) -> Self {
        Value {
            ir: ir.into(),
            is_unsigned,
        }
        // match is_unsigned {
        //     true => Value::UnsignedInt(ir),
        //     false => Value::SignedInt(ir),
        // }
    }
}

impl<'ctx> Deref for Value<'ctx> {
    type Target = BasicValueEnum<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.ir
    }
}
