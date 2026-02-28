use inkwell::builder::Builder;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::boolean_value::BooleanValue;
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
    ) -> Value<'ctx> {
        match operation {
            BinaryOperation::Add => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_int_add(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::Sub => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_int_sub(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::Mul => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_int_mul(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::Div => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_int_unsigned_div(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::Mod => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_int_unsigned_rem(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::BitAnd => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_and(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::BitXor => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_xor(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::BitOr => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_or(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::ShiftLeft => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_left_shift(self.ir, arg.to_ir().into_int_value(), "")
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::ShiftRight => Value::IntegerValue(IntegerValue {
                ir: builder
                    .build_right_shift(
                        self.ir,
                        arg.to_ir().into_int_value(),
                        self.value_type.is_signed,
                        "",
                    )
                    .unwrap(),
                value_type: self.value_type,
            }),

            BinaryOperation::Eq => Value::BooleanValue(BooleanValue {
                ir: builder
                    .build_int_compare(
                        inkwell::IntPredicate::EQ,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),
            }),

            BinaryOperation::Ne => Value::BooleanValue(BooleanValue {
                ir: builder
                    .build_int_compare(
                        inkwell::IntPredicate::NE,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),
            }),

            BinaryOperation::Gt => Value::BooleanValue(BooleanValue {
                ir: builder
                    .build_int_compare(
                        inkwell::IntPredicate::UGT,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),
            }),

            BinaryOperation::Ge => Value::BooleanValue(BooleanValue {
                ir: builder
                    .build_int_compare(
                        inkwell::IntPredicate::UGE,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),
            }),

            BinaryOperation::Lt => Value::BooleanValue(BooleanValue {
                ir: builder
                    .build_int_compare(
                        inkwell::IntPredicate::ULT,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),
            }),

            BinaryOperation::Le => Value::BooleanValue(BooleanValue {
                ir: builder
                    .build_int_compare(
                        inkwell::IntPredicate::ULE,
                        self.ir,
                        arg.to_ir().into_int_value(),
                        "",
                    )
                    .unwrap(),
            }),
        }
    }

    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}
