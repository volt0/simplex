use inkwell::context::Context;
use inkwell::types::BasicTypeEnum;

use crate::errors::{CompilationError, CompilationResult};
use crate::float_type::FloatType;
use crate::function_type::FunctionType;
use crate::integer_type::IntegerType;
use crate::module_builder::ModuleBuilder;

#[derive(Clone)]
pub enum TypeSpec {
    Reference(String),
}

pub type BoolTypeIR<'ctx> = inkwell::types::IntType<'ctx>;

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
        Ok(match type_spec {
            TypeSpec::Reference(name) => module_builder.load_type(&name)?,
        })
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
        Type::Float(FloatType::new_f32(context))
    }

    #[inline]
    pub fn new_f64(context: &'ctx Context) -> Self {
        Type::Float(FloatType::new_f64(context))
    }

    #[inline]
    pub fn new_bool(context: &'ctx Context) -> Self {
        Type::Bool(context.bool_type())
    }
}

impl<'ctx> TryInto<BasicTypeEnum<'ctx>> for Type<'ctx> {
    type Error = CompilationError;

    fn try_into(self) -> Result<BasicTypeEnum<'ctx>, Self::Error> {
        Ok(match self {
            Type::Integer(int_type) => BasicTypeEnum::IntType(int_type.into()),
            Type::Float(float_type) => BasicTypeEnum::FloatType(float_type.into()),
            Type::Bool(ir) => BasicTypeEnum::IntType(ir),
            _ => return Err(CompilationError::InvalidOperation),
        })
    }
}
