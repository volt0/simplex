use inkwell::builder::Builder;
use inkwell::values::{BasicValue, BasicValueEnum};

use crate::expression::{BinaryOperation, UnaryOperation};
use crate::type_spec::{FloatType, TypeSpec};
use crate::value::Value;

type FloatValueIR<'ctx> = inkwell::values::FloatValue<'ctx>;

#[derive(Clone)]
pub struct FloatValue<'ctx> {
    pub ir: FloatValueIR<'ctx>,
    pub value_type: FloatType,
}

impl<'ctx> FloatValue<'ctx> {
    pub fn type_check(self, type_hint: &TypeSpec) -> Value<'ctx> {
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
