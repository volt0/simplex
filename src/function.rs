use std::collections::HashMap;
use std::ops::Deref;

use inkwell::builder::Builder;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{AnyValue, BasicMetadataValueEnum, BasicValueEnum};

use crate::ast;
use crate::block::{Block, BlockVisitor};
use crate::errors::{CompilationError, CompilationResult};
use crate::module::ModuleBuilder;
use crate::statement::StatementTranslator;
use crate::types::Type;
use crate::values::Value;

type FunctionIR<'ctx> = inkwell::values::FunctionValue<'ctx>;
type FunctionTypeIR<'ctx> = inkwell::types::FunctionType<'ctx>;

#[derive(Clone)]
pub struct Function<'ctx> {
    ir: FunctionIR<'ctx>,
    func_type: FunctionType<'ctx>,
}

impl<'ctx> Function<'ctx> {
    #[inline]
    pub fn from_ir(ir: FunctionIR<'ctx>, func_type: FunctionType<'ctx>) -> Self {
        Self { ir, func_type }
    }

    #[inline]
    pub fn get_type(&self) -> &FunctionType<'ctx> {
        &self.func_type
    }

    #[inline]
    pub fn get_return_type(&self) -> &Type<'ctx> {
        self.func_type.return_type()
    }

    #[inline]
    pub fn ir(&self) -> &FunctionIR<'ctx> {
        &self.ir
    }

    pub fn do_call(
        &self,
        builder: &Builder<'ctx>,
        args: &[Value<'ctx>],
    ) -> CompilationResult<Value<'ctx>> {
        let mut args_ir: Vec<BasicMetadataValueEnum> = Vec::with_capacity(args.len());
        for arg in args {
            let arg: BasicValueEnum = arg.clone().try_into()?;
            args_ir.push(arg.into());
        }

        let result_ir = builder.build_call(self.ir, args_ir.as_slice(), "")?;
        Value::from_ir(result_ir.as_any_value_enum(), self.get_return_type())
    }
}

#[derive(Clone, PartialEq)]
pub struct FunctionType<'ctx> {
    ir: FunctionTypeIR<'ctx>,
    arg_types: Vec<Type<'ctx>>,
    return_type: Box<Type<'ctx>>,
}

impl<'ctx> FunctionType<'ctx> {
    pub fn from_ast(
        module_builder: &ModuleBuilder<'ctx>,
        signature: &ast::FunctionSignature,
    ) -> CompilationResult<Self> {
        let args_count = signature.args.len();
        let mut arg_types = Vec::with_capacity(args_count);
        let mut arg_types_ir = Vec::with_capacity(args_count);
        for arg_type in signature.args.iter() {
            let arg_type = Type::from_spec(module_builder, arg_type.value_type.clone())?;
            arg_types.push(arg_type.clone());
            let arg_type_ir: BasicTypeEnum = arg_type.try_into()?;
            arg_types_ir.push(arg_type_ir.into());
        }

        let return_type = Type::from_spec(module_builder, signature.return_type.clone())?;
        let return_type_ir: BasicTypeEnum = return_type.clone().try_into()?;
        let func_type_ir = return_type_ir.fn_type(&arg_types_ir, false);

        Ok(FunctionType {
            ir: func_type_ir,
            return_type: Box::new(return_type),
            arg_types,
        })
    }

    pub fn validate_value(&self, value: &Value<'ctx>) -> CompilationResult<Function<'ctx>> {
        match value {
            Value::Function(value) if value.get_type() == self => Ok(value.clone()),
            _ => Err(CompilationError::TypeMismatch),
        }
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
}

pub struct FunctionBuilder<'ctx, 'm> {
    parent: &'m mut ModuleBuilder<'ctx>,
    builder: Builder<'ctx>,
    func: Function<'ctx>,
    func_args: HashMap<String, Value<'ctx>>,
}

impl<'ctx, 'm> FunctionBuilder<'ctx, 'm> {
    pub fn new(
        func: Function<'ctx>,
        func_signature: ast::FunctionSignature,
        parent: &'m mut ModuleBuilder<'ctx>,
    ) -> CompilationResult<Self> {
        let mut func_builder = Self {
            func,
            func_args: HashMap::with_capacity(func_signature.args.len()),
            builder: parent.context().create_builder(),
            parent,
        };

        for arg_ast in func_signature.args.into_iter() {
            func_builder.add_argument(arg_ast.name.clone(), arg_ast)?;
        }

        Ok(func_builder)
    }

    fn add_argument(
        &mut self,
        name: String,
        arg_ast: ast::FunctionArgument,
    ) -> CompilationResult<()> {
        let func_ir = self.function_ir();
        let arg_id = self.func_args.len() as u32;
        let arg_ir = func_ir.get_nth_param(arg_id).unwrap().as_any_value_enum();
        let arg_type = Type::from_spec(self, arg_ast.value_type)?;
        self.func_args
            .insert(name, Value::from_ir(arg_ir, &arg_type)?);

        Ok(())
    }

    pub fn attach_body(&self, body: Block) -> CompilationResult<()> {
        let body_ir = self
            .context()
            .append_basic_block(self.function_ir().clone(), "");

        self.builder().position_at_end(body_ir);

        let stmt_translator = StatementTranslator::new(self);
        stmt_translator.enter_block(&body)
    }

    #[inline(always)]
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    #[inline(always)]
    pub fn function_return_type(&self) -> &Type<'ctx> {
        self.func.get_return_type()
    }

    #[inline(always)]
    pub fn function_ir(&self) -> &FunctionIR<'ctx> {
        self.func.ir()
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.func_args.get(name) {
            Some(arg) => Ok(arg.clone()),
            None => self.parent.load_value(name),
        }
    }

    pub fn build(self) -> Function<'ctx> {
        self.func
    }
}

impl<'ctx, 'm> Deref for FunctionBuilder<'ctx, 'm> {
    type Target = ModuleBuilder<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}
