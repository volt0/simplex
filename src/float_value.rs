use inkwell::values::AnyValueEnum;

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::float_type::FloatType;
use crate::value::Value;

type FloatValueIR<'ctx> = inkwell::values::FloatValue<'ctx>;

#[derive(Clone)]
#[repr(transparent)]
pub struct FloatValue<'ctx> {
    pub ir: FloatValueIR<'ctx>,
}

impl<'ctx> Into<Value<'ctx>> for FloatValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Float(self)
    }
}

impl<'ctx> FloatValue<'ctx> {
    pub fn from_value(
        value: &Value<'ctx>,
        expr_type: &FloatType,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        Ok(match value {
            Value::Float(value) => value.extend_to(expr_type, expr_translator)?,
            Value::Integer(value) => value.to_float(expr_translator)?,
            _ => return Err(CompilationError::TypeMismatch),
        })
    }

    pub fn from_ir(ir: AnyValueEnum<'ctx>, _: &FloatType) -> Self {
        if let AnyValueEnum::FloatValue(ir) = ir {
            return FloatValue { ir };
        }
        panic!("Expected FloatValue, got {:?}", ir);
    }

    pub fn into_ir(self) -> AnyValueEnum<'ctx> {
        AnyValueEnum::FloatValue(self.ir)
    }

    pub fn value_type(&self) -> FloatType {
        match self.ir.get_type().get_bit_width() {
            32 => FloatType::F32,
            64 => FloatType::F64,
            bit_width => panic!("Unsupported float bit width: {}", bit_width),
        }
    }

    pub fn binary_operation(
        &self,
        op: BinaryOperation,
        other: &Value<'ctx>,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs_ir = self.ir;
        let rhs_ir = match other {
            Value::Float(other) => other.ir,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let builder = expr_translator.builder();
        let result_ir = match op {
            BinaryOperation::Add => builder.build_float_add(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Sub => builder.build_float_sub(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Mul => builder.build_float_mul(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Div => builder.build_float_div(lhs_ir, rhs_ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };

        Ok(Value::Float(FloatValue { ir: result_ir }))
    }

    pub fn unary_operation(
        &self,
        op: UnaryOperation,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = expr_translator.builder();
        let result_ir = match op {
            UnaryOperation::Plus => self.ir.clone(),
            UnaryOperation::Minus => builder.build_float_neg(self.ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };

        Ok(Value::Float(FloatValue { ir: result_ir }))
    }

    fn extend_to(
        &self,
        target_type: &FloatType,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        let context = expr_translator.context();
        if &self.value_type() <= target_type {
            Ok(FloatValue {
                ir: expr_translator.builder().build_float_ext(
                    self.ir,
                    target_type.to_ir(context),
                    "",
                )?,
            })
        } else {
            Err(CompilationError::TypeMismatch)
        }
    }
}
