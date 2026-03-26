use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for Value<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Integer(value) => value.into(),
        }
    }
}

pub struct IntegerValue<'ctx>(IntValue<'ctx>);

impl<'ctx> IntegerValue<'ctx> {
    pub fn from_constant(value: i32, context: &'ctx Context) -> Self {
        Self(context.i32_type().const_int(value as u64, true))
    }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        self.0.as_basic_value_enum()
    }
}
