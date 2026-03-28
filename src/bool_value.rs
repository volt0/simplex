use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::errors::CompilationError;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::integer_value::IntegerValue;
use crate::value::Value;

#[derive(Clone)]
pub struct BoolValue<'ctx> {
    pub(crate) ir: IntValue<'ctx>,
}

impl<'ctx> Into<Value<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Bool(self)
    }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}

impl<'ctx> BoolValue<'ctx> {
    pub fn to_integer(
        &self,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Result<IntegerValue<'ctx>, CompilationError> {
        Ok(IntegerValue {
            ir: builder.build_int_z_extend(self.ir, context.i8_type(), "")?,
            value_type: IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I8,
            },
        })
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Result<Value<'ctx>, CompilationError> {
        let other = match other {
            Value::Bool(other) => other,
            Value::Integer(other) => &other.to_bool(builder, context)?,
            Value::Bool(other) => other.clone(),
            Value::Integer(other) => other.to_bool(builder, context)?,
        };

        let lhs_ir = self.ir;
        let rhs_ir = other.ir;
        Ok(BoolValue {
            ir: match operation {
                BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, "")?,
                BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, "")?,
                BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, "")?,
                _ => return Err(CompilationError::InvalidOperation),
            },
        }
        .into())
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
    ) -> Result<Value<'ctx>, CompilationError> {
        Ok(BoolValue {
            ir: match operation {
                UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
                _ => return Err(CompilationError::InvalidOperation),
            },
        }
        .into())
    }
}
