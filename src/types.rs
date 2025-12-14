use crate::ast;
use crate::function::FunctionSignature;
use crate::integer::{IntegerType, IntegerTypeSize};
use crate::module::ModuleTranslator;
use inkwell::types::FunctionType as FunctionTypeIR;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use std::ops::Deref;

pub type TypeHint = TypeSpec;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSpec {
    Void,
    Primitive(PrimitiveType),
    Function(Box<FunctionType>),
}

impl TypeSpec {
    pub fn from_ast(type_spec_ast: &ast::Type) -> Self {
        match type_spec_ast {
            ast::Type::Integer(int_type) => {
                TypeSpec::Primitive(PrimitiveType::Integer(IntegerType::from_ast(int_type)))
            }
            ast::Type::Identifier(_) => todo!(),
            ast::Type::Void => TypeSpec::Primitive(PrimitiveType::Void),
            ast::Type::Boolean => TypeSpec::Primitive(PrimitiveType::Bool),
            ast::Type::Float(float_type) => {
                TypeSpec::Primitive(PrimitiveType::Float(FloatType::from_ast(float_type)))
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveType {
    Void,
    Bool,
    Integer(IntegerType),
    Float(FloatType),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum FloatType {
    F32,
}

impl FloatType {
    pub fn from_ast(float_type_ast: &ast::FloatType) -> Self {
        match float_type_ast {
            ast::FloatType::F32 => FloatType::F32,
            ast::FloatType::F64 => FloatType::F32,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType {
    pub arg_types: Vec<TypeSpec>,
    pub return_type: TypeSpec,
}

impl FunctionType {
    pub fn new(function_signature: &FunctionSignature) -> Box<Self> {
        let mut function_type = Box::new(FunctionType {
            arg_types: Vec::with_capacity(function_signature.args.len()),
            return_type: function_signature.return_type.clone(),
        });

        for arg in &function_signature.args {
            function_type.arg_types.push(arg.arg_type.clone())
        }

        function_type
    }
}

#[repr(transparent)]
pub struct TypeTranslator<'ctx, 'm> {
    parent: &'m ModuleTranslator<'ctx>,
}

impl<'ctx, 'm> Deref for TypeTranslator<'ctx, 'm> {
    type Target = ModuleTranslator<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> TypeTranslator<'ctx, 'm> {
    pub fn new(parent: &'m ModuleTranslator<'ctx>) -> Self {
        TypeTranslator { parent }
    }

    pub fn translate_type(&self, type_spec: &TypeSpec) -> BasicTypeEnum<'ctx> {
        match type_spec {
            TypeSpec::Primitive(primitive_type) => self.translate_primitive_type(primitive_type),
            TypeSpec::Function(_) => todo!(),
            TypeSpec::Void => todo!(),
        }
    }

    pub fn translate_primitive_type(&self, primitive_type: &PrimitiveType) -> BasicTypeEnum<'ctx> {
        let context = self.context;
        match primitive_type {
            PrimitiveType::Void => todo!(),
            PrimitiveType::Bool => context.bool_type().as_basic_type_enum(),
            PrimitiveType::Integer(integer_type) => {
                let type_ir = match integer_type.width {
                    IntegerTypeSize::I8 => context.i8_type(),
                    IntegerTypeSize::I16 => context.i16_type(),
                    IntegerTypeSize::I32 => context.i32_type(),
                    IntegerTypeSize::I64 => context.i64_type(),
                };
                type_ir.as_basic_type_enum()
            }
            PrimitiveType::Float(float_type) => {
                let type_ir = match float_type {
                    FloatType::F32 => context.f32_type(),
                };
                type_ir.as_basic_type_enum()
            }
        }
    }

    pub fn translate_function_type(&self, function_type: &FunctionType) -> FunctionTypeIR<'ctx> {
        let arg_type_irs: Vec<BasicMetadataTypeEnum> = function_type
            .arg_types
            .iter()
            .map(|arg_type| self.translate_type(&arg_type).into())
            .collect();

        let return_type = &function_type.return_type;
        match return_type {
            TypeSpec::Void => {
                let void_type_ir = self.context.void_type();
                void_type_ir.fn_type(&arg_type_irs, false)
            }
            return_type => {
                let return_type_ir = self.translate_type(&return_type);
                return_type_ir.fn_type(&arg_type_irs, false)
            }
        }
    }
}
