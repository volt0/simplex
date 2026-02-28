use inkwell::builder::Builder;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_value::IntegerValue;
use crate::type_spec::TypeSpec;
use crate::value::Value;

#[derive(Clone)]
#[repr(transparent)]
pub struct BooleanValue<'ctx> {
    pub ir: IntValue<'ctx>,
}

impl<'ctx> BooleanValue<'ctx> {
    pub fn type_check(self, type_hint: &TypeSpec) -> IntegerValue<'ctx> {
        todo!()
    }

    pub fn unary_operation(
        self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
    ) -> Value<'ctx> {
        todo!()
    }

    pub fn binary_operation(
        self,
        operation: BinaryOperation,
        arg: Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Value<'ctx> {
        todo!()
    }

    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}
