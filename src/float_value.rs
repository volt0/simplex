use inkwell::values::{BasicValue, BasicValueEnum};

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

impl<'ctx> Into<BasicValueEnum<'ctx>> for FloatValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}

impl<'ctx> FloatValue<'ctx> {
    pub fn from_value(
        value: &Value<'ctx>,
        expression_type: &FloatType,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        let builder = expression_translator.builder();
        let context = expression_translator.context();
        Ok(match value {
            Value::Float(value) => {
                if &value.value_type <= expression_type {
                    FloatValue {
                        ir: builder.build_float_ext(
                            value.ir,
                            expression_type.to_ir(context),
                            "",
                        )?,
                        value_type: expression_type.clone(),
                    }
                } else {
                    return Err(CompilationError::TypeMismatch);
                }
            }
            Value::Integer(value) => value.to_float(expression_translator)?,
            _ => return Err(CompilationError::TypeMismatch),
        })
    }

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
}
