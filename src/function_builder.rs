use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use inkwell::builder::Builder;
use inkwell::values::{AnyValue, FunctionValue};

use crate::ast;
use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::function::Function;
use crate::module_builder::ModuleBuilder;
use crate::statement_translator::StatementTranslator;
use crate::types::Type;
use crate::value::Value;

#[derive(Clone)]
pub struct FunctionArgument<'ctx> {
    pub name: String,
    pub value: Value<'ctx>,
}

pub struct FunctionBuilder<'ctx, 'm> {
    func: Function<'ctx>,
    func_args: HashMap<String, FunctionArgument<'ctx>>,
    parent: &'m mut ModuleBuilder<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionBuilder<'ctx, 'm> {
    type Target = ModuleBuilder<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> DerefMut for FunctionBuilder<'ctx, 'm> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> FunctionBuilder<'ctx, 'm> {
    pub fn new(
        func: Function<'ctx>,
        func_signature: &ast::FunctionSignature,
        parent: &'m mut ModuleBuilder<'ctx>,
    ) -> CompilationResult<Self> {
        let mut func_args = HashMap::new();
        let func_ir = func.ir();
        for (i, arg) in func_signature.args.iter().enumerate() {
            let arg_ir = func_ir.get_nth_param(i as u32).unwrap().as_any_value_enum();
            let arg_type = Type::from_spec(parent.context(), arg.value_type.clone());
            func_args.insert(
                arg.name.clone(),
                FunctionArgument {
                    name: arg.name.clone(),
                    value: Value::from_ir(arg_ir, &arg_type)?,
                },
            );
        }

        let func = Function::new(func_ir.clone(), func.get_type().clone());

        let builder = parent.context().create_builder();
        let func_builder = Self {
            func,
            func_args,
            parent,
            builder,
        };

        Ok(func_builder)
    }

    pub fn attach_body(&self, body: BasicBlock) -> CompilationResult<()> {
        let body_ir = self
            .context()
            .append_basic_block(self.function_ir().clone(), "");

        self.builder().position_at_end(body_ir);
        let stmt_translator = StatementTranslator::new(self);
        body.visit(&stmt_translator)
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
    pub fn function_ir(&self) -> &FunctionValue<'ctx> {
        self.func.ir()
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.func_args.get(name) {
            Some(arg) => Ok(arg.value.clone()),
            None => self.parent.load_value(name),
        }
    }

    pub fn build(self) -> Function<'ctx> {
        self.func
    }
}
