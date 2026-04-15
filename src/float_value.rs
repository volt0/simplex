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

    pub fn get_type(&self) -> FloatType<'ctx> {
        FloatType::new(self.ir.get_type())
    }

    pub fn binary_operation(
        self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let other = match other {
            Value::Float(other) => other,
            Value::Integer(other) => other.to_float(builder, &self.get_type())?,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let lhs_ir = self.ir;
        let rhs_ir = other.ir;
        let result_ir = match op {
            BinaryOperation::Add => builder.build_float_add(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Sub => builder.build_float_sub(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Mul => builder.build_float_mul(lhs_ir, rhs_ir, "")?,
            BinaryOperation::Div => builder.build_float_div(lhs_ir, rhs_ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(Self { ir: result_ir }.into())
    }

    pub fn unary_operation(
        self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<Value<'ctx>> {
        let result_ir = match op {
            UnaryOperation::Plus => self.ir.clone(),
            UnaryOperation::Minus => builder.build_float_neg(self.ir, "")?,
            _ => return Err(CompilationError::InvalidOperation),
        };
        Ok(Self { ir: result_ir }.into())
    }

    pub fn extend(
        self,
        builder: &Builder<'ctx>,
        target_type: &FloatType<'ctx>,
    ) -> CompilationResult<Self> {
        let self_type_ir = self.ir.get_type();
        if self_type_ir.get_bit_width() > target_type.bit_width() {
            return Err(CompilationError::TypeMismatch);
        }

        let result_type_ir = target_type.ir();
        let result_ir = builder.build_float_ext(self.ir, result_type_ir.clone(), "")?;
        Ok(Self { ir: result_ir })
    }
}
