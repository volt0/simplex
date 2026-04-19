use std::ops::{Deref, DerefMut};

use inkwell::builder::Builder;
use inkwell::values::{AnyValue, FunctionValue};

use crate::ast;
use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::function::Function;
use crate::function_type::FunctionType;
use crate::module_builder::ModuleBuilder;
use crate::statement_translator::StatementTranslator;
use crate::types::Type;
use crate::value::Value;

pub struct FunctionBuilder<'ctx, 'm> {
    func: Function<'ctx>,
    func_signature: ast::FunctionSignature,
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
        func_type: FunctionType<'ctx>,
        func_ir: FunctionValue<'ctx>,
        func_signature: &ast::FunctionSignature,
        parent: &'m mut ModuleBuilder<'ctx>,
    ) -> CompilationResult<Self> {
        let mut func = Function::new(func_ir.clone(), func_type);

        for (i, arg) in func_signature.args.iter().enumerate() {
            let arg_ir = func_ir.get_nth_param(i as u32).unwrap().as_any_value_enum();
            let arg_type = Type::from_spec(parent.context(), arg.value_type.clone());
            func.args
                .insert(arg.name.clone(), Value::from_ir(arg_ir, &arg_type)?);
        }

        let builder = parent.context().create_builder();
        let func_builder = Self {
            func,
            func_signature: func_signature.clone(),
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
    pub fn function_signature(&self) -> &ast::FunctionSignature {
        &self.func_signature
    }

    #[inline(always)]
    pub fn function_ir(&self) -> &FunctionValue<'ctx> {
        self.func.ir()
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.func.args.get(name) {
            Some(value) => Ok(value.clone()),
            None => self.parent.load_value(name),
        }
    }

    pub fn build(self) -> Function<'ctx> {
        self.func
    }
}
