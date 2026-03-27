use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::expression::BinaryOperation;

pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn into_ir(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Integer(value) => value.into(),
        }
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Self {
        match self {
            Value::Integer(value) => match other {
                Value::Integer(other) => value.binary_operation(operation, other, builder).into(),
            },
        }
    }
}

pub struct IntegerValue<'ctx> {
    ir: IntValue<'ctx>,
}

impl<'ctx> Into<Value<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Integer(self)
    }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn from_constant(value: i32, context: &'ctx Context) -> Self {
        IntegerValue {
            ir: context.i32_type().const_int(value as u64, true),
        }
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &IntegerValue<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Self {
        let lhs_ir = self.ir;
        let rhs_ir = other.ir;
        let result_ir = match operation {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, "").unwrap(),
        };

        IntegerValue { ir: result_ir }
    }
}
