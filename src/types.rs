use crate::ast;
use crate::function::FunctionSignature;
use crate::module::ModuleCompiler;

use inkwell::types::{
    BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType as FunctionTypeIr,
};
use std::ops::Deref;

#[derive(Clone, Debug, PartialEq)]
pub enum Type {
    Void,
    Primitive(PrimitiveType),
    Function(Box<FunctionType>),
}

impl Type {
    pub fn from_ast(type_spec_ast: &ast::Type) -> Self {
        match type_spec_ast {
            ast::Type::Integer(int_type) => {
                Type::Primitive(PrimitiveType::Integer(IntegerType::from_ast(int_type)))
            }
            ast::Type::Identifier(_) => todo!(),
            ast::Type::Void => Type::Primitive(PrimitiveType::Void),
            ast::Type::Boolean => Type::Primitive(PrimitiveType::Bool),
            ast::Type::Float(float_type) => {
                Type::Primitive(PrimitiveType::Float(FloatType::from_ast(float_type)))
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

#[derive(Clone, Debug, PartialEq)]
pub struct IntegerType {
    pub is_signed: bool,
    pub width: IntegerTypeSize,
}

impl IntegerType {
    pub fn from_ast(int_type_ast: &ast::IntegerType) -> Self {
        match int_type_ast {
            ast::IntegerType::I8 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I8,
            },
            ast::IntegerType::I16 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I16,
            },
            ast::IntegerType::I32 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I32,
            },
            ast::IntegerType::I64 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I64,
            },
            ast::IntegerType::U8 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I8,
            },
            ast::IntegerType::U16 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I16,
            },
            ast::IntegerType::U32 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I32,
            },
            ast::IntegerType::U64 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I64,
            },
        }
    }
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
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
    pub arg_types: Vec<Type>,
    pub return_type: Type,
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

#[derive(Debug, PartialEq)]
pub enum TypeHint {
    Integer(IntegerType),
    Inferred,
}

impl TypeHint {
    pub fn from_ast(type_spec_ast: Option<&ast::Type>) -> Self {
        match type_spec_ast {
            None => TypeHint::Inferred,
            Some(type_ast) => match type_ast {
                ast::Type::Identifier(_) => todo!(),
                ast::Type::Void => todo!(),
                ast::Type::Boolean => todo!(),
                ast::Type::Integer(int_type_ast) => {
                    TypeHint::Integer(IntegerType::from_ast(int_type_ast))
                }
                ast::Type::Float(_) => todo!(),
            },
        }
    }
}

#[repr(transparent)]
pub struct TypeCompiler<'ctx, 'm> {
    parent: &'m ModuleCompiler<'ctx>,
}

impl<'ctx, 'm> Deref for TypeCompiler<'ctx, 'm> {
    type Target = ModuleCompiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> TypeCompiler<'ctx, 'm> {
    pub fn new(parent: &'m ModuleCompiler<'ctx>) -> Self {
        TypeCompiler { parent }
    }

    pub fn compile_type(&self, type_spec: &Type) -> BasicTypeEnum<'ctx> {
        match type_spec {
            Type::Primitive(primitive_type) => self.compile_primitive_type(primitive_type),
            Type::Function(_) => todo!(),
            Type::Void => todo!(),
        }
    }

    pub fn compile_primitive_type(&self, primitive_type: &PrimitiveType) -> BasicTypeEnum<'ctx> {
        let context = self.backend_context;
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

    pub fn compile_function_type(&self, function_type: &FunctionType) -> FunctionTypeIr<'ctx> {
        let arg_type_irs: Vec<BasicMetadataTypeEnum> = function_type
            .arg_types
            .iter()
            .map(|arg_type| self.compile_type(&arg_type).into())
            .collect();

        let return_type = &function_type.return_type;
        match return_type {
            Type::Void => {
                let void_type_ir = self.backend_context.void_type();
                void_type_ir.fn_type(&arg_type_irs, false)
            }
            return_type => {
                let return_type_ir = self.compile_type(&return_type);
                return_type_ir.fn_type(&arg_type_irs, false)
            }
        }
    }
}
