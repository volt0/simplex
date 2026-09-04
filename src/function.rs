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
    function_ir: FunctionIR<'ctx>,
    function_type: FunctionType<'ctx>,
    args: HashMap<String, Value<'ctx>>,
}

impl<'ctx> Function<'ctx> {
    pub fn new(
        function_ir: FunctionIR<'ctx>,
        function_type: FunctionType<'ctx>,
    ) -> CompilationResult<Self> {
        let mut args = HashMap::with_capacity(function_type.arg_types.len());
        for (i, (arg_name, arg_type)) in function_type.arg_types.iter().enumerate() {
            let arg_ir = function_ir
                .get_nth_param(i as u32)
                .unwrap()
                .as_any_value_enum();

            args.insert(arg_name.clone(), Value::from_ir(arg_ir, &arg_type)?);
        }

        Ok(Function {
            function_ir,
            function_type,
            args,
        })
    }

    #[inline]
    pub fn function_type(&self) -> &FunctionType<'ctx> {
        &self.function_type
    }

    #[inline]
    pub fn return_type(&self) -> &Type<'ctx> {
        self.function_type.return_type()
    }

    #[inline]
    pub fn ir(&self) -> &FunctionIR<'ctx> {
        &self.function_ir
    }

    pub fn call(
        &self,
        builder: &Builder<'ctx>,
        args: &[Value<'ctx>],
    ) -> CompilationResult<Value<'ctx>> {
        let mut args_ir: Vec<BasicMetadataValueEnum> = Vec::with_capacity(args.len());
        for arg in args {
            let arg: BasicValueEnum = arg.clone().try_into()?;
            args_ir.push(arg.into());
        }

        let result_ir = builder.build_call(self.function_ir, args_ir.as_slice(), "")?;
        Value::from_ir(result_ir.as_any_value_enum(), self.return_type())
    }
}

#[derive(Clone, PartialEq)]
pub struct FunctionType<'ctx> {
    ir: FunctionTypeIR<'ctx>,
    arg_types: Vec<(String, Type<'ctx>)>,
    return_type: Box<Type<'ctx>>,
}

impl<'ctx> FunctionType<'ctx> {
    pub fn from_ast(
        signature: &ast::FunctionSignature,
        module_builder: &ModuleBuilder<'ctx>,
    ) -> CompilationResult<Self> {
        let args_count = signature.args.len();
        let mut arg_types = Vec::with_capacity(args_count);
        let mut arg_types_ir = Vec::with_capacity(args_count);
        for arg in signature.args.iter() {
            let arg_type = Type::from_spec(module_builder, arg.value_type.clone())?;
            arg_types.push((arg.name.clone(), arg_type.clone()));

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
            Value::Function(value) if value.function_type() == self => Ok(value.clone()),
            _ => Err(CompilationError::TypeMismatch),
        }
    }

    #[inline(always)]
    pub fn arg_types(&self) -> &[(String, Type<'ctx>)] {
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
    function: Function<'ctx>,
    module_builder: &'m mut ModuleBuilder<'ctx>,
    ir_builder: Builder<'ctx>,
}

impl<'ctx, 'm> FunctionBuilder<'ctx, 'm> {
    pub fn new(
        function: Function<'ctx>,
        module_builder: &'m mut ModuleBuilder<'ctx>,
    ) -> CompilationResult<Self> {
        let context = module_builder.context();
        Ok(Self {
            ir_builder: context.create_builder(),
            function,
            module_builder,
        })
    }

    pub fn attach_body(&self, body: Block) -> CompilationResult<()> {
        let context = self.context();
        let func_ir = self.function_ir();
        let body_ir = context.append_basic_block(func_ir.clone(), "");

        self.ir_builder.position_at_end(body_ir);

        let stmt_translator = StatementTranslator::new(self);
        stmt_translator.enter_block(&body)
    }

    #[inline(always)]
    pub fn ir_builder(&self) -> &Builder<'ctx> {
        &self.ir_builder
    }

    #[inline(always)]
    pub fn function_return_type(&self) -> &Type<'ctx> {
        self.function.return_type()
    }

    #[inline(always)]
    pub fn function_ir(&self) -> &FunctionIR<'ctx> {
        self.function.ir()
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.function.args.get(name) {
            Some(arg) => Ok(arg.clone()),
            None => self.module_builder.load_value(name),
        }
    }

    pub fn build(self) -> Function<'ctx> {
        self.function
    }
}

impl<'ctx, 'm> Deref for FunctionBuilder<'ctx, 'm> {
    type Target = ModuleBuilder<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.module_builder
    }
}
