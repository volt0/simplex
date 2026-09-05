use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use crate::ast;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::FunctionType;
use crate::module::ModuleBuilder;
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

pub struct TypeTranslator<'ctx, 'm> {
    module_builder: &'m ModuleBuilder<'ctx>,
}

impl<'ctx, 'm> TypeTranslator<'ctx, 'm> {
    pub fn new(module_builder: &'m ModuleBuilder<'ctx>) -> Self {
        Self { module_builder }
    }

    pub fn create_type(&self, type_spec: TypeSpec) -> CompilationResult<Type<'ctx>> {
        match type_spec {
            TypeSpec::Reference(name) => self.module_builder.load_type(&name),
        }
    }

    pub fn create_function_type(
        &self,
        signature: &ast::FunctionSignature,
    ) -> CompilationResult<FunctionType<'ctx>> {
        let mut arg_types = Vec::with_capacity(signature.args.len());
        for arg in signature.args.iter() {
            let arg_type = self.create_type(arg.value_type.clone())?;
            arg_types.push((arg.name.clone(), arg_type.clone()));
        }

        let return_type = self.create_type(signature.return_type.clone())?;

        FunctionType::new(arg_types, return_type)
    }
}
