use inkwell::builder::{Builder, BuilderError};
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::boolean_value::BooleanValue;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::type_spec::IntegerType;
use crate::type_spec::TypeSpec;
use crate::value::Value;

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub ir: IntValue<'ctx>,
    pub value_type: IntegerType,
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn type_check(self, type_hint: &TypeSpec) -> Value<'ctx> {
        match type_hint {
            TypeSpec::Integer(integer_type) if integer_type == &self.value_type => self.into(),
            _ => panic!("Type mismatch"),
        }
    }

    pub fn unary_operation(
        self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
    ) -> Result<Value<'ctx>, BuilderError> {
        Ok(match operation {
            UnaryOperation::Plus => self.into(),

            UnaryOperation::Minus => IntegerValue {
                ir: builder.build_int_neg(self.ir, "")?,
                value_type: self.value_type,
            }
            .into(),

            UnaryOperation::BitNot => IntegerValue {
                ir: builder.build_not(self.ir, "")?,
                value_type: self.value_type,
            }
            .into(),
        })
    }

    pub fn binary_operation(
        self,
        operation: BinaryOperation,
        arg: Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Result<Value<'ctx>, BuilderError> {
        Ok(match operation {
            BinaryOperation::Add => IntegerValue {
                ir: builder.build_int_add(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::Sub => IntegerValue {
                ir: builder.build_int_sub(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::Mul => IntegerValue {
                ir: builder.build_int_mul(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::Div => IntegerValue {
                ir: builder.build_int_unsigned_div(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::Mod => IntegerValue {
                ir: builder.build_int_unsigned_rem(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::BitAnd => IntegerValue {
                ir: builder.build_and(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::BitXor => IntegerValue {
                ir: builder.build_xor(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::BitOr => IntegerValue {
                ir: builder.build_or(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::ShiftLeft => IntegerValue {
                ir: builder.build_left_shift(self.ir, arg.to_ir().into_int_value(), "")?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::ShiftRight => IntegerValue {
                ir: builder.build_right_shift(
                    self.ir,
                    arg.to_ir().into_int_value(),
                    self.value_type.is_signed,
                    "",
                )?,
                value_type: self.value_type,
            }
            .into(),

            BinaryOperation::Eq => BooleanValue {
                ir: builder.build_int_compare(
                    inkwell::IntPredicate::EQ,
                    self.ir,
                    arg.to_ir().into_int_value(),
                    "",
                )?,
            }
            .into(),

            BinaryOperation::Ne => BooleanValue {
                ir: builder.build_int_compare(
                    inkwell::IntPredicate::NE,
                    self.ir,
                    arg.to_ir().into_int_value(),
                    "",
                )?,
            }
            .into(),

            BinaryOperation::Gt => BooleanValue {
                ir: builder.build_int_compare(
                    inkwell::IntPredicate::UGT,
                    self.ir,
                    arg.to_ir().into_int_value(),
                    "",
                )?,
            }
            .into(),

            BinaryOperation::Ge => BooleanValue {
                ir: builder.build_int_compare(
                    inkwell::IntPredicate::UGE,
                    self.ir,
                    arg.to_ir().into_int_value(),
                    "",
                )?,
            }
            .into(),

            BinaryOperation::Lt => BooleanValue {
                ir: builder.build_int_compare(
                    inkwell::IntPredicate::ULT,
                    self.ir,
                    arg.to_ir().into_int_value(),
                    "",
                )?,
            }
            .into(),

            BinaryOperation::Le => BooleanValue {
                ir: builder.build_int_compare(
                    inkwell::IntPredicate::ULE,
                    self.ir,
                    arg.to_ir().into_int_value(),
                    "",
                )?,
            }
            .into(),
        })
    }

    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}
