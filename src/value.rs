use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::float_value::FloatValue;
use crate::function_value::FunctionValue;
use crate::integer_value::IntegerValue;
use crate::types::Type;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    Bool(BoolValue<'ctx>),
    Function(FunctionValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(value_ir: AnyValueEnum<'ctx>, value_type: &Type) -> CompilationResult<Self> {
        Ok(match value_type {
            Type::Integer(value_type) => {
                IntegerValue::new(value_ir.into_int_value(), value_type.is_signed()).into()
            }
            Type::Float(_) => FloatValue::new(value_ir.into_float_value()).into(),
            Type::Bool => BoolValue::new(value_ir.into_int_value()).into(),
        })
    }
}

impl<'ctx> TryInto<BasicValueEnum<'ctx>> for Value<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicValueEnum<'ctx>, Self::Error> {
        Ok(match self {
            Value::Integer(value) => BasicValueEnum::IntValue(value.into()),
            Value::Bool(value) => BasicValueEnum::IntValue(value.into()),
            Value::Float(value) => BasicValueEnum::FloatValue(value.into()),
            _ => return Err(CompilationError::InvalidOperation),
        })
    }
}
