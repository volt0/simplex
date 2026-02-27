use inkwell::values::{BasicValue, BasicValueEnum};

use crate::expression::UnaryOperation;

pub type IntegerValueIR<'ctx> = inkwell::values::IntValue<'ctx>;

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub(crate) ir: IntegerValueIR<'ctx>,
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn unary_operation(self, operation: UnaryOperation) -> IntegerValue<'ctx> {
        match operation {
            UnaryOperation::Plus => self,
            _ => todo!(),
            // UnaryOperation::Minus => self.const_neg().as_basic_value_enum(),
            // UnaryOperation::BitNot => self.const_not().as_basic_value_enum(),
            // UnaryOperation::LogicalNot => {
            //     self.const_int(0, false).const_ne(self).as_basic_value_enum()
            // }
        }
    }

    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}
