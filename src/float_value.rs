use inkwell::values::AnyValueEnum;

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::float_type::FloatType;
use crate::value::Value;

type FloatValueIR<'ctx> = inkwell::values::FloatValue<'ctx>;
type FloatTypeIR<'ctx> = inkwell::types::FloatType<'ctx>;

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

impl<'ctx> Into<FloatValueIR<'ctx>> for FloatValue<'ctx> {
    fn into(self) -> FloatValueIR<'ctx> {
        self.ir
    }
}

impl<'ctx> FloatValue<'ctx> {
    pub fn new(ir: AnyValueEnum<'ctx>) -> Self {
        if let AnyValueEnum::FloatValue(ir) = ir {
            return FloatValue { ir };
        }
        panic!("Expected FloatValue, got {:?}", ir);
    }

    pub fn from_value(
        value: &Value<'ctx>,
        expr_type: &FloatType,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        let expr_type = FloatValueType::new(expr_type, expr_translator);
        Ok(match value {
            Value::Float(value) => value.extend_to(expr_type, expr_translator)?,
            Value::Integer(value) => value.to_float(expr_translator)?,
            _ => return Err(CompilationError::TypeMismatch),
        })
    }

    #[inline(always)]
    pub fn type_of(&self) -> FloatValueType<'ctx> {
        FloatValueType {
            ir: self.ir.get_type(),
        }
    }

    pub fn value_type(&self) -> FloatType {
        self.type_of().into()
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
        target_type: FloatValueType<'ctx>,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<Self> {
        let self_width = self.ir.get_type().get_bit_width();
        let target_width = target_type.ir.get_bit_width();
        if self_width <= target_width {
            let builder = expr_translator.builder();
            let result_ir = builder.build_float_ext(self.ir, target_type.ir, "")?;
            Ok(FloatValue { ir: result_ir })
        } else {
            Err(CompilationError::TypeMismatch)
        }
    }
}

#[repr(transparent)]
pub struct FloatValueType<'ctx> {
    pub ir: FloatTypeIR<'ctx>,
}

impl<'ctx> Into<FloatType> for FloatValueType<'ctx> {
    fn into(self) -> FloatType {
        match self.ir.get_bit_width() {
            32 => FloatType::F32,
            64 => FloatType::F64,
            width => panic!("Invalid float type width: {}", width),
        }
    }
}

impl<'ctx> FloatValueType<'ctx> {
    pub fn new(
        type_spec: &FloatType,
        expr_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> Self {
        let context = expr_translator.context();
        FloatValueType {
            ir: match type_spec {
                FloatType::F32 => context.f32_type(),
                FloatType::F64 => context.f64_type(),
            },
        }
    }
}
