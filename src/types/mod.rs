use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use crate::errors::{CompilationError, CompilationResult};
use crate::function::FunctionType;
use crate::module_builder::ModuleBuilder;
use crate::types::boolean::BoolType;
use crate::values::Value;

use floating::FloatType;
use integer::IntType;

pub mod boolean;
pub mod floating;
pub mod integer;

#[derive(Clone)]
pub enum TypeSpec {
    Reference(String),
}

#[derive(Clone, PartialEq)]
pub enum Type<'ctx> {
    Int(IntType<'ctx>),
    Float(FloatType<'ctx>),
    Bool(BoolType<'ctx>),
    Function(FunctionType<'ctx>),
}

impl<'ctx> Type<'ctx> {
    pub fn from_spec(
        module_builder: &ModuleBuilder<'ctx>,
        type_spec: TypeSpec,
    ) -> CompilationResult<Self> {
        match type_spec {
            TypeSpec::Reference(name) => module_builder.load_type(&name),
        }
    }

    pub fn validate_value(
        &self,
        builder: &Builder<'ctx>,
        value: &Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let value = match self {
            Self::Int(required_type) => required_type.validate_value(builder, value)?.into(),
            Self::Float(required_type) => required_type.validate_value(builder, value)?.into(),
            Self::Bool(required_type) => required_type.validate_value(builder, value)?.into(),
            Self::Function(required_type) => required_type.validate_value(value)?.into(),
        };
        Ok(value)
    }

    #[inline]
    pub fn new_i8(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Int(IntType::new_i8(context, is_signed))
    }

    #[inline]
    pub fn new_i16(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Int(IntType::new_i16(context, is_signed))
    }

    #[inline]
    pub fn new_i32(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Int(IntType::new_i32(context, is_signed))
    }

    #[inline]
    pub fn new_i64(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Int(IntType::new_i64(context, is_signed))
    }

    #[inline]
    pub fn new_f32(context: &'ctx Context) -> Self {
        Self::Float(FloatType::new_f32(context))
    }

    #[inline]
    pub fn new_f64(context: &'ctx Context) -> Self {
        Self::Float(FloatType::new_f64(context))
    }

    #[inline]
    pub fn new_bool(context: &'ctx Context) -> Self {
        Self::Bool(BoolType::new(context))
    }
}

impl<'ctx> TryInto<BasicTypeEnum<'ctx>> for Type<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicTypeEnum<'ctx>, Self::Error> {
        match self {
            Self::Int(int_type) => Ok(int_type.into()),
            Self::Float(float_type) => Ok(float_type.into()),
            Self::Bool(bool_type) => Ok(bool_type.into()),
            _ => Err(CompilationError::InvalidOperation),
        }
    }
}
