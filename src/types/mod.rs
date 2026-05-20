use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use crate::errors::{CompilationError, CompilationResult};
use crate::function::FunctionType;
use crate::module_builder::ModuleBuilder;
use crate::values::Value;

use floating::FloatType;
use integer::IntegerType;
use primitive::PrimitiveType;

pub mod boolean;
pub mod floating;
pub mod integer;
pub mod primitive;

#[derive(Clone)]
pub enum TypeSpec {
    Reference(String),
}

#[derive(Clone, PartialEq)]
pub enum Type<'ctx> {
    Primitive(PrimitiveType<'ctx>),
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
        match self {
            Type::Primitive(required_type) => {
                let result = required_type.check_value(builder, &value.to_primitive()?)?;
                Ok(result.into())
            }
            Type::Function(required_type) => {
                if value.get_type() == Type::Function(required_type.clone()) {
                    Ok(value.clone())
                } else {
                    Err(CompilationError::TypeMismatch)
                }
            }
        }
    }

    #[inline]
    pub fn new_i8(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Primitive(PrimitiveType::Integer(IntegerType::new_i8(
            context, is_signed,
        )))
    }

    #[inline]
    pub fn new_i16(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Primitive(PrimitiveType::Integer(IntegerType::new_i16(
            context, is_signed,
        )))
    }

    #[inline]
    pub fn new_i32(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Primitive(PrimitiveType::Integer(IntegerType::new_i32(
            context, is_signed,
        )))
    }

    #[inline]
    pub fn new_i64(context: &'ctx Context, is_signed: bool) -> Self {
        Self::Primitive(PrimitiveType::Integer(IntegerType::new_i64(
            context, is_signed,
        )))
    }

    #[inline]
    pub fn new_f32(context: &'ctx Context) -> Self {
        Self::Primitive(PrimitiveType::Float(FloatType::new_f32(context)))
    }

    #[inline]
    pub fn new_f64(context: &'ctx Context) -> Self {
        Self::Primitive(PrimitiveType::Float(FloatType::new_f64(context)))
    }

    #[inline]
    pub fn new_bool(context: &'ctx Context) -> Self {
        Self::Primitive(PrimitiveType::Bool(context.bool_type()))
    }
}

impl<'ctx> TryInto<BasicTypeEnum<'ctx>> for Type<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicTypeEnum<'ctx>, Self::Error> {
        match self {
            Type::Primitive(prim_type) => prim_type.try_into(),
            _ => Err(CompilationError::InvalidOperation),
        }
    }
}
