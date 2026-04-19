use std::collections::HashMap;
use std::ops::{Deref, DerefMut};

use inkwell::builder::Builder;
use inkwell::values::{AnyValue, FunctionValue};

use crate::ast;
use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::function::FunctionVisitor;
use crate::module_builder::ModuleBuilder;
use crate::statement_translator::StatementTranslator;
use crate::types::Type;
use crate::value::Value;

pub struct FunctionTranslator<'ctx, 'm> {
    func_signature: ast::FunctionSignature,
    func_ir: FunctionValue<'ctx>,
    args_ir: HashMap<String, Value<'ctx>>,
    parent: &'m mut ModuleBuilder<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionTranslator<'ctx, 'm> {
    type Target = ModuleBuilder<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> DerefMut for FunctionTranslator<'ctx, 'm> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> FunctionVisitor for FunctionTranslator<'ctx, 'm> {
    fn visit_body(&self, body: &BasicBlock) -> CompilationResult<()> {
        let body_ir = self.context().append_basic_block(self.func_ir.clone(), "");
        self.builder().position_at_end(body_ir);

        let stmt_translator = StatementTranslator::new(self);
        body.visit(&stmt_translator)
    }
}

impl<'ctx, 'm> FunctionTranslator<'ctx, 'm> {
    pub fn new(
        func_ir: FunctionValue<'ctx>,
        func_signature: &ast::FunctionSignature,
        parent: &'m mut ModuleBuilder<'ctx>,
    ) -> CompilationResult<Self> {
        let mut args_ir = HashMap::with_capacity(func_ir.count_params() as usize);
        for (i, arg) in func_signature.args.iter().enumerate() {
            let arg_ir = func_ir.get_nth_param(i as u32).unwrap().as_any_value_enum();
            let arg_type = Type::from_spec(parent.context(), arg.value_type.clone());
            args_ir.insert(arg.name.clone(), Value::from_ir(arg_ir, &arg_type)?);
        }

        let builder = parent.context().create_builder();
        Ok(Self {
            func_signature: func_signature.clone(),
            func_ir,
            args_ir,
            parent,
            builder,
        })
    }

    #[inline(always)]
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    #[inline(always)]
    pub fn function_signature(&self) -> &ast::FunctionSignature {
        &self.func_signature
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.args_ir.get(name) {
            Some(value) => Ok(value.clone()),
            None => self.parent.load_value(name),
        }
    }
}
