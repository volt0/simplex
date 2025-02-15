use super::module_compiler::ModuleCompiler;
use crate::function::Function;
use crate::types::Type;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use std::ops::Deref;

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

    pub fn compile_function_type(&self, function: &Function) -> FunctionType<'ctx> {
        let return_type = function.return_type();
        let return_type_ir = self.compile_type(&return_type);

        let arg_type_irs: Vec<BasicMetadataTypeEnum> = function
            .iter_args()
            .map(|arg| {
                let arg_type = arg.arg_type();
                self.compile_type(&arg_type).into()
            })
            .collect();

        return_type_ir.fn_type(&arg_type_irs, false)
    }

    pub fn compile_type(&self, type_spec: &Type) -> BasicTypeEnum<'ctx> {
        let context = self.context();
        match type_spec {
            Type::I64 => context.i64_type().as_basic_type_enum(),
        }
    }
}
