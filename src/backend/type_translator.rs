use std::ops::Deref;

use inkwell::types::{
    BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType as FunctionTypeIR,
};

use super::module_translator::ModuleTranslator;
use crate::types::{FloatType, FunctionType, IntegerTypeSize, TypeSpec};

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
        let context = self.context;
        match type_spec {
            TypeSpec::Void => todo!(),
            TypeSpec::Bool => context.bool_type().as_basic_type_enum(),
            TypeSpec::Integer(integer_type) => {
                let type_ir = match integer_type.width {
                    IntegerTypeSize::I8 => context.i8_type(),
                    IntegerTypeSize::I16 => context.i16_type(),
                    IntegerTypeSize::I32 => context.i32_type(),
                    IntegerTypeSize::I64 => context.i64_type(),
                };
                type_ir.as_basic_type_enum()
            }
            TypeSpec::Float(float_type) => {
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

        match &function_type.return_type {
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
