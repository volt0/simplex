use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use super::integer::{IntegerType, IntegerValue};
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::types::Type;
use crate::values::Value;

type BoolTypeIR<'ctx> = inkwell::types::IntType<'ctx>;

#[derive(Clone, PartialEq)]
pub struct BoolType<'ctx>(BoolTypeIR<'ctx>);

impl<'ctx> BoolType<'ctx> {
    #[inline]
    pub fn from_ir(ir: BoolTypeIR<'ctx>) -> Self {
        Self(ir)
    }

    #[inline]
    pub fn new(context: &'ctx Context) -> Self {
        Self(context.bool_type())
    }

    pub fn validate_value(
        &self,
        builder: &Builder<'ctx>,
        value: &Value<'ctx>,
    ) -> CompilationResult<BoolValue<'ctx>> {
        match value {
            Value::Bool(value) => Ok(value.clone()),
            Value::Integer(value) => value.to_bool(builder),
            _ => Err(CompilationError::TypeMismatch),
        }
    }
}

impl<'ctx> Into<BasicTypeEnum<'ctx>> for BoolType<'ctx> {
    fn into(self) -> BasicTypeEnum<'ctx> {
        BasicTypeEnum::IntType(self.0.clone())
    }
}

impl<'ctx> Into<Type<'ctx>> for BoolType<'ctx> {
    fn into(self) -> Type<'ctx> {
        Type::Bool(self)
    }
}

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

    pub fn get_type(&self) -> BoolType<'ctx> {
        BoolType::from_ir(self.ir.get_type())
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
        &self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let other = self.get_type().validate_value(builder, other)?;
        let lhs_ir = self.ir;
        let rhs_ir = other.ir;
        let result_ir = match op {
            BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, "")?,
            BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, "")?,
            BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(BoolValue::from_ir(result_ir).into())
    }

    pub fn do_unary_operation(
        &self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<Value<'ctx>> {
        let result_ir = match op {
            UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(BoolValue::from_ir(result_ir).into())
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
