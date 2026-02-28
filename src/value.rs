use inkwell::builder::Builder;
use inkwell::values::BasicValueEnum;

use crate::boolean_value::BooleanValue;
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::float_value::FloatValue;
use crate::integer_value::IntegerValue;
use crate::type_spec::TypeSpec;

#[derive(Clone)]
pub enum Value<'ctx> {
    IntegerValue(IntegerValue<'ctx>),
    BooleanValue(BooleanValue<'ctx>),
    FloatValue(FloatValue<'ctx>),
}

impl<'ctx> From<IntegerValue<'ctx>> for Value<'ctx> {
    fn from(value: IntegerValue<'ctx>) -> Self {
        Value::IntegerValue(value)
    }
}

impl<'ctx> From<BooleanValue<'ctx>> for Value<'ctx> {
    fn from(value: BooleanValue<'ctx>) -> Self {
        Value::BooleanValue(value)
    }
}

impl<'ctx> From<FloatValue<'ctx>> for Value<'ctx> {
    fn from(value: FloatValue<'ctx>) -> Self {
        Value::FloatValue(value)
    }
}

impl<'ctx> Value<'ctx> {
    pub fn type_check(self, type_hint: &TypeSpec) -> Value<'ctx> {
        match self {
            Value::IntegerValue(value) => value.type_check(type_hint),
            Value::BooleanValue(value) => value.type_check(type_hint),
            Value::FloatValue(value) => value.type_check(type_hint),
        }
    }

    pub fn unary_operation(
        self,
        operation: UnaryOperation,
        builder: &Builder<'ctx>,
    ) -> Value<'ctx> {
        match self {
            Value::IntegerValue(value) => value.unary_operation(operation, builder).unwrap(),
            Value::BooleanValue(value) => value.unary_operation(operation, builder),
            Value::FloatValue(value) => value.unary_operation(operation, builder),
        }
    }

    pub fn binary_operation(
        self,
        operation: BinaryOperation,
        arg: Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Value<'ctx> {
        match self {
            Value::IntegerValue(value) => value.binary_operation(operation, arg, builder).unwrap(),
            Value::BooleanValue(value) => value.binary_operation(operation, arg, builder),
            Value::FloatValue(value) => value.binary_operation(operation, arg, builder),
        }
    }

    pub fn get_type(&self) -> TypeSpec {
        match self {
            Value::IntegerValue(value) => TypeSpec::Integer(value.value_type.clone()),
            Value::FloatValue(value) => TypeSpec::Float(value.value_type.clone()),
            Value::BooleanValue(_) => TypeSpec::Boolean,
        }
    }

    pub fn to_ir(&self) -> BasicValueEnum<'ctx> {
        match self {
            Value::IntegerValue(value) => value.to_ir(),
            Value::BooleanValue(value) => value.to_ir(),
            Value::FloatValue(value) => value.to_ir(),
        }
    }
}
