use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::integer_value::IntegerValue;
use crate::types::Type;
use crate::value::Value;

#[derive(Clone)]
pub struct BoolValue<'ctx> {
    pub ir: IntValue<'ctx>,
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
    pub fn from_ir(value_ir: BasicValueEnum<'ctx>) -> CompilationResult<Self> {
        if let BasicValueEnum::IntValue(value_ir) = value_ir {
            return Ok(BoolValue { ir: value_ir });
        }
        panic!("Expected BoolValue, got {:?}", value_ir);
    }

    pub fn to_integer(
        &self,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
        expression_type: Option<&Type>,
    ) -> CompilationResult<IntegerValue<'ctx>> {
        let value_type = match expression_type {
            None => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I8,
            },
            Some(Type::Integer(expression_type)) => expression_type.clone(),
            _ => unreachable!(),
        };

        Ok(IntegerValue {
            ir: builder.build_int_z_extend(self.ir, value_type.to_ir(context), "")?,
            value_type,
        })
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        if let Some(expression_type) = expression_type {
            if !matches!(expression_type, &Type::Bool) {
                return Err(CompilationError::TypeMismatch);
            }
        }

        let other = match other {
            Value::Bool(other) => other.clone(),
            Value::Integer(other) => other.to_bool(builder, context)?,
            _ => return Err(CompilationError::TypeMismatch),
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
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        if let Some(expression_type) = expression_type {
            if !matches!(expression_type, &Type::Bool) {
                return Err(CompilationError::TypeMismatch);
            }
        }

        Ok(BoolValue {
            ir: match operation {
                UnaryOperation::BitNot => builder.build_not(self.ir, "")?,
                _ => return Err(CompilationError::InvalidOperation),
            },
        }
        .into())
    }
}
