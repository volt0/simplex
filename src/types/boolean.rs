use inkwell::builder::Builder;

use super::integer::{IntegerType, IntegerValue};
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::types::Type;
use crate::values::Value;

pub type BoolTypeIR<'ctx> = inkwell::types::IntType<'ctx>;

type BoolValueIR<'ctx> = inkwell::values::IntValue<'ctx>;

#[derive(Clone)]
#[repr(transparent)]
pub struct BoolValue<'ctx> {
    ir: BoolValueIR<'ctx>,
}

impl<'ctx> BoolValue<'ctx> {
    #[inline]
    pub fn from_ir(ir: BoolValueIR<'ctx>) -> Self {
        BoolValue { ir }
    }

    pub fn get_type(&self) -> Type<'ctx> {
        Type::Bool(self.ir.get_type())
    }

    pub fn to_integer(
        &self,
        builder: &Builder<'ctx>,
        required_type: &IntegerType<'ctx>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        let value_type_ir = required_type.ir().clone();
        let value_ir = builder.build_int_z_extend(self.ir, value_type_ir, "")?;
        Ok(IntegerValue::from_ir(value_ir, required_type.is_signed()))
    }

    pub fn do_binary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &BoolValue<'ctx>,
    ) -> CompilationResult<()> {
        let lhs_ir = self.ir;
        let rhs_ir = other.ir;
        self.ir = match op {
            BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, "")?,
            BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, "")?,
            BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(())
    }

    pub fn do_unary_operation(
        &mut self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<()> {
        self.ir = match op {
            UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(())
    }
}

impl<'ctx> Into<BoolValueIR<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> BoolValueIR<'ctx> {
        self.ir
    }
}

impl<'ctx> Into<Value<'ctx>> for BoolValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Bool(self)
    }
}
