use inkwell::builder::Builder;
use inkwell::values::IntValue;

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_type::IntegerType;
use crate::integer_value::IntegerValue;
use crate::value::Value;

#[derive(Clone)]
pub struct BoolValue<'ctx> {
    ir: IntValue<'ctx>,
}

impl<'ctx> Into<Value<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Bool(self)
    }
}

impl<'ctx> Into<IntValue<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> IntValue<'ctx> {
        self.ir
    }
}

impl<'ctx> BoolValue<'ctx> {
    pub fn new(ir: IntValue<'ctx>) -> Self {
        BoolValue { ir }
    }

    pub fn to_integer(
        &self,
        value_type: &IntegerType<'ctx>,
        builder: &Builder<'ctx>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        let value_type_ir = value_type.ir().clone();
        let value_ir = builder.build_int_z_extend(self.ir, value_type_ir, "")?;
        Ok(IntegerValue::new(value_ir, value_type.is_signed()))
    }

    pub fn binary_operation(
        &self,
        op: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs_ir = self.ir;
        let rhs_ir = match other {
            Value::Bool(other) => other.ir,
            _ => return Err(CompilationError::TypeMismatch),
        };

        Ok(BoolValue {
            ir: match op {
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
        op: UnaryOperation,
        builder: &Builder<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        Ok(BoolValue {
            ir: match op {
                UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
                _ => return Err(CompilationError::InvalidOperation),
            },
        }
        .into())
    }
}
