use std::ops::Deref;

use inkwell::values::{AnyValueEnum, BasicValueEnum};

use crate::errors::{CompilationError, CompilationResult};
use crate::function::Function;
use crate::types::Type;

use self::bool_value::BoolValue;

mod bool_value;
mod float_value;
mod integer_value;
mod value_operations;

pub use self::float_value::FloatValue;
pub use self::integer_value::IntegerValue;
pub use self::value_operations::ValueOperations;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
    Float(FloatValue<'ctx>),
    Bool(BoolValue<'ctx>),
    Function(Function<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(
        value_ir: AnyValueEnum<'ctx>,
        value_type: &Type<'ctx>,
    ) -> CompilationResult<Self> {
        Ok(match value_type {
            Type::Integer(value_type) => {
                IntegerValue::new(value_ir.into_int_value(), value_type.is_signed()).into()
            }
            Type::Float(_) => FloatValue::new(value_ir.into_float_value()).into(),
            Type::Bool(_) => BoolValue::new(value_ir.into_int_value()).into(),
            Type::Function(value_type) => {
                Function::new(value_ir.into_function_value(), value_type.clone()).into()
            }
        })
    }
}

impl<'ctx> Deref for Value<'ctx> {
    type Target = dyn ValueOperations<'ctx> + 'ctx;

    fn deref(&self) -> &Self::Target {
        match self {
            Value::Integer(value) => value,
            Value::Float(value) => value,
            Value::Bool(value) => value,
            Value::Function(value) => value,
        }
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
