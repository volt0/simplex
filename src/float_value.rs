use inkwell::values::AnyValueEnum;

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::float_type::FloatType;
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

impl<'ctx> FloatValue<'ctx> {
    pub fn from_value(
        value: &Value<'ctx>,
        expression_type: &FloatType,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        Ok(match value {
            Value::Float(value) => value.extend_to(expression_type, expression_translator)?,
            Value::Integer(value) => value.to_float(expression_translator)?,
            _ => return Err(CompilationError::TypeMismatch),
        })
    }

    pub fn from_ir(value_ir: AnyValueEnum<'ctx>, value_type: &FloatType) -> Self {
        if let AnyValueEnum::FloatValue(value_ir) = value_ir {
            return FloatValue {
                ir: value_ir,
                value_type: value_type.clone(),
            };
        }
        panic!("Expected FloatValue, got {:?}", value_ir);
    }

    pub fn into_ir(self) -> AnyValueEnum<'ctx> {
        AnyValueEnum::FloatValue(self.ir)
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs_ir = self.ir;
        let rhs_ir = match other {
            Value::Float(other) => other.ir,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let builder = expression_translator.builder();
        let result_ir = match operation {
            BinaryOperation::Add => builder.build_float_add(lhs_ir, rhs_ir, ""),
            BinaryOperation::Sub => builder.build_float_sub(lhs_ir, rhs_ir, ""),
            BinaryOperation::Mul => builder.build_float_mul(lhs_ir, rhs_ir, ""),
            BinaryOperation::Div => builder.build_float_div(lhs_ir, rhs_ir, ""),
            _ => return Err(CompilationError::InvalidOperation),
        };

        Ok(Value::Float(FloatValue {
            ir: result_ir?,
            value_type: self.value_type.clone(),
        }))
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = expression_translator.builder();
        let result_ir = match operation {
            UnaryOperation::Plus => self.ir.clone(),
            UnaryOperation::Minus => builder.build_float_neg(self.ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };

        Ok(Value::Float(FloatValue {
            ir: result_ir,
            value_type: self.value_type.clone(),
        }))
    }

    fn extend_to(
        &self,
        target_type: &FloatType,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        let context = expression_translator.context();
        if &self.value_type <= target_type {
            Ok(FloatValue {
                ir: expression_translator.builder().build_float_ext(
                    self.ir,
                    target_type.to_ir(context),
                    "",
                )?,
                value_type: target_type.clone(),
            })
        } else {
            Err(CompilationError::TypeMismatch)
        }
    }
}
