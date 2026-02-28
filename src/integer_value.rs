use inkwell::builder::Builder;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_type::IntegerType;
use crate::type_spec::TypeSpec;
use crate::value::Value;

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub ir: IntValue<'ctx>,
    pub value_type: IntegerType,
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn type_check(self, type_hint: &TypeSpec) -> IntegerValue<'ctx> {
        match type_hint {
            TypeSpec::Integer(integer_type) if integer_type == &self.value_type => self,
            _ => panic!("Type mismatch"),
        }
    }

    pub fn unary_operation(
        self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
    ) -> IntegerValue<'ctx> {
        match operation {
            UnaryOperation::Plus => self,

            UnaryOperation::Minus => IntegerValue {
                ir: builder.build_int_neg(self.ir, "").unwrap(),
                value_type: self.value_type,
            },

            UnaryOperation::BitNot => IntegerValue {
                ir: builder.build_not(self.ir, "").unwrap(),
                value_type: self.value_type,
            },
        }
    }

    pub fn binary_operation(
        self,
        operation: BinaryOperation,
        arg: Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> IntegerValue<'ctx> {
        IntegerValue {
            ir: match operation {
                BinaryOperation::Add => builder
                    .build_int_add(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::Sub => builder
                    .build_int_sub(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::Mul => builder
                    .build_int_mul(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::Div => builder
                    .build_int_unsigned_div(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::Mod => builder
                    .build_int_unsigned_rem(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::BitAnd => builder
                    .build_and(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::BitXor => builder
                    .build_xor(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::BitOr => builder
                    .build_or(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::ShiftLeft => builder
                    .build_left_shift(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),

                BinaryOperation::ShiftRight => builder
                    .build_right_shift(self.ir, arg.to_ir().into_int_value(), true, "")
                    .unwrap(),

                BinaryOperation::Eq => builder
                    .build_int_compare(
                        inkwell::IntPredicate::EQ,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),

                BinaryOperation::Ne => builder
                    .build_int_compare(
                        inkwell::IntPredicate::NE,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),

                BinaryOperation::Gt => builder
                    .build_int_compare(
                        inkwell::IntPredicate::UGT,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),

                BinaryOperation::Ge => builder
                    .build_int_compare(
                        inkwell::IntPredicate::UGE,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),

                BinaryOperation::Lt => builder
                    .build_int_compare(
                        inkwell::IntPredicate::ULT,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),

                BinaryOperation::Le => builder
                    .build_int_compare(
                        inkwell::IntPredicate::ULE,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),
            },
            value_type: self.value_type,
        }
    }

    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}
