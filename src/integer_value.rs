use inkwell::builder::Builder;
use inkwell::values::{BasicValue, BasicValueEnum};

use crate::expression::UnaryOperation;

pub type IntegerValueIR<'ctx> = inkwell::values::IntValue<'ctx>;

#[derive(Clone)]
#[repr(transparent)]
pub struct IntegerValue<'ctx>(pub IntegerValueIR<'ctx>);

impl<'ctx> IntegerValue<'ctx> {
    pub fn unary_operation(
        self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
    ) -> IntegerValue<'ctx> {
        match operation {
            UnaryOperation::Plus => self,
            UnaryOperation::Minus => IntegerValue(builder.build_int_neg(self.0, "").unwrap()),
            UnaryOperation::BitNot => IntegerValue(builder.build_not(self.0, "").unwrap()),
            UnaryOperation::LogicalNot => todo!(),
        }
    }

    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        self.0.as_basic_value_enum()
    }
}
