use inkwell::builder::Builder;

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::float_type::FloatType;
use crate::value::Value;

type FloatValueIR<'ctx> = inkwell::values::FloatValue<'ctx>;

#[derive(Clone)]
#[repr(transparent)]
pub struct FloatValue<'ctx> {
    ir: FloatValueIR<'ctx>,
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
    pub fn new(ir: FloatValueIR<'ctx>) -> Self {
        FloatValue { ir }
    }

    pub fn binary_operation(
        &self,
        op: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs_ir = self.ir;
        let rhs_ir = match other {
            Value::Float(other) => other.ir,
            _ => return Err(CompilationError::TypeMismatch),
        };

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
        builder: &Builder<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let result_ir = match op {
            UnaryOperation::Plus => self.ir.clone(),
            UnaryOperation::Minus => builder.build_float_neg(self.ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };

        Ok(Value::Float(FloatValue { ir: result_ir }))
    }

    pub fn extend(
        &self,
        target_type: &FloatType<'ctx>,
        builder: &Builder<'ctx>,
    ) -> CompilationResult<Self> {
        let self_type_ir = self.ir.get_type();
        if self_type_ir.get_bit_width() <= target_type.bit_width() {
            let result_type_ir = target_type.ir();
            let result_ir = builder.build_float_ext(self.ir, result_type_ir.clone(), "")?;
            Ok(FloatValue { ir: result_ir })
        } else {
            Err(CompilationError::TypeMismatch)
        }
    }
}
