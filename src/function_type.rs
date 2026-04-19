use inkwell::context::Context;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};

use crate::ast;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::Function;
use crate::types::Type;
use crate::value::Value;

pub type FunctionTypeIR<'ctx> = inkwell::types::FunctionType<'ctx>;

#[derive(Clone, PartialEq)]
pub struct FunctionType<'ctx> {
    ir: FunctionTypeIR<'ctx>,
    arg_types: Vec<Type<'ctx>>,
    return_type: Box<Type<'ctx>>,
}

impl<'ctx> Into<Type<'ctx>> for FunctionType<'ctx> {
    fn into(self) -> Type<'ctx> {
        Type::Function(self)
    }
}

impl<'ctx> FunctionType<'ctx> {
    pub fn from_ast(
        context: &'ctx Context,
        signature: &ast::FunctionSignature,
    ) -> CompilationResult<Self> {
        let mut arg_types = Vec::with_capacity(signature.args.len());
        let mut arg_types_ir = Vec::<BasicMetadataTypeEnum>::new();
        for arg_type in signature.args.iter() {
            let arg_type = Type::from_spec(context, arg_type.value_type.clone());
            arg_types.push(arg_type.clone());
            let arg_type_ir: BasicTypeEnum = arg_type.try_into()?;
            arg_types_ir.push(arg_type_ir.into());
        }

        let return_type = Type::from_spec(context, signature.return_type.clone());
        let return_type_ir: BasicTypeEnum = return_type.clone().try_into()?;
        let func_type_ir = return_type_ir.fn_type(&arg_types_ir, false);

        Ok(FunctionType {
            ir: func_type_ir,
            return_type: Box::new(return_type),
            arg_types,
        })
    }

    #[inline(always)]
    pub fn arg_types(&self) -> &[Type<'ctx>] {
        &self.arg_types
    }

    #[inline(always)]
    pub fn return_type(&self) -> &Type<'ctx> {
        self.return_type.as_ref()
    }

    #[inline(always)]
    pub fn ir(&self) -> &FunctionTypeIR<'ctx> {
        &self.ir
    }

    pub fn validate_value(self, value: Value<'ctx>) -> CompilationResult<Function<'ctx>> {
        match value {
            Value::Function(value) if value.get_type() == &self => Ok(value),
            _ => Err(CompilationError::TypeMismatch),
        }
    }
}
