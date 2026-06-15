use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use crate::errors::{CompilationError, CompilationResult};
use crate::function::FunctionType;
use crate::module_builder::ModuleBuilder;
use crate::types::boolean::BoolTypeIR;
use crate::values::Value;

use floating::FloatType;
use integer::IntegerType;

pub mod boolean;
pub mod floating;
pub mod integer;

#[derive(Clone)]
pub enum TypeSpec {
    Reference(String),
}

#[derive(Clone, PartialEq)]
pub enum Type<'ctx> {
    Integer(IntegerType<'ctx>),
    Float(FloatType<'ctx>),
    Bool(BoolTypeIR<'ctx>),
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

    pub fn check_value(
        &self,
        builder: &Builder<'ctx>,
        value: Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let value = match self {
            Self::Integer(required_type) => value.to_int(builder, required_type)?.into(),
            Self::Float(required_type) => value.to_float(builder, required_type)?.into(),
            Self::Bool(_) => value.to_bool(builder)?.into(),
            Self::Function(required_type) => {
                if value.get_type() == Type::Function(required_type.clone()) {
                    value.clone()
                } else {
                    return Err(CompilationError::TypeMismatch);
                }
            }
        };
        Ok(value)
    }

    #[inline]
    pub fn new_i8(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Integer(IntegerType::new_i8(context, is_signed))
    }

    #[inline]
    pub fn new_i16(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Integer(IntegerType::new_i16(context, is_signed))
    }

    #[inline]
    pub fn new_i32(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Integer(IntegerType::new_i32(context, is_signed))
    }

    #[inline]
    pub fn new_i64(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Integer(IntegerType::new_i64(context, is_signed))
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
        Self::Bool(context.bool_type())
    }
}

impl<'ctx> TryInto<BasicTypeEnum<'ctx>> for Type<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicTypeEnum<'ctx>, Self::Error> {
        match self {
            Self::Integer(int_type) => Ok(int_type.into()),
            Self::Float(float_type) => Ok(float_type.into()),
            Self::Bool(ir) => Ok(BasicTypeEnum::IntType(ir)),
            _ => Err(CompilationError::InvalidOperation),
        }
    }
}
