use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::{BasicValue, BasicValueEnum};

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::float_type::FloatType;
use crate::types::Type;
use crate::value::Value;

type FloatValueIR<'ctx> = inkwell::values::FloatValue<'ctx>;

#[derive(Clone)]
pub struct FloatValue<'ctx> {
    pub ir: FloatValueIR<'ctx>,
    pub value_type: FloatType,
}

impl<'ctx> Into<Value<'ctx>> for FloatValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Float(self)
    }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for FloatValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}

impl<'ctx> FloatValue<'ctx> {
    pub fn from_ir(
        value_ir: BasicValueEnum<'ctx>,
        value_type: &FloatType,
    ) -> CompilationResult<Self> {
        if let BasicValueEnum::FloatValue(value_ir) = value_ir {
            return Ok(FloatValue {
                ir: value_ir,
                value_type: value_type.clone(),
            });
        }
        panic!("Expected FloatValue, got {:?}", value_ir);
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let other = match other {
            Value::Float(other) => other.clone(),
            Value::Integer(other) => other.to_float(builder, context)?,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let lhs_type = self.value_type.clone();
        let rhs_type = other.value_type.clone();
        let result_type = match expression_type {
            None => {
                if rhs_type > lhs_type {
                    rhs_type
                } else {
                    lhs_type
                }
            }
            Some(Type::Float(expression_type)) => expression_type.clone(),
            _ => unreachable!(),
        };

        let result_type_ir = result_type.to_ir(context);
        let lhs_ir = builder.build_float_ext(self.ir, result_type_ir, "")?;
        let rhs_ir = builder.build_float_ext(other.ir, result_type_ir, "")?;

        let result_ir = match operation {
            BinaryOperation::Add => builder.build_float_add(lhs_ir, rhs_ir, ""),
            BinaryOperation::Sub => builder.build_float_sub(lhs_ir, rhs_ir, ""),
            BinaryOperation::Mul => builder.build_float_mul(lhs_ir, rhs_ir, ""),
            BinaryOperation::Div => builder.build_float_div(lhs_ir, rhs_ir, ""),
            _ => return Err(CompilationError::InvalidOperation),
        };

        Ok(Value::Float(FloatValue {
            ir: result_ir?,
            value_type: result_type,
        }))
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let result_type = match expression_type {
            None => self.value_type.clone(),
            Some(Type::Float(expression_type)) => expression_type.clone(),
            _ => unreachable!(),
        };

        let arg_ir = builder.build_float_ext(self.ir, result_type.to_ir(context), "")?;
        let result_ir = match operation {
            UnaryOperation::Plus => Ok(arg_ir),
            UnaryOperation::Minus => builder.build_float_neg(arg_ir, ""),
            _ => return Err(CompilationError::InvalidOperation),
        };

        Ok(Value::Float(FloatValue {
            ir: result_ir?,
            value_type: result_type,
        }))
    }
}
