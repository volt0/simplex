use std::collections::HashMap;
use std::ops::Deref;

use inkwell::builder::Builder;
use inkwell::values::{AnyValue, FunctionValue};

use crate::basic_block::BasicBlock;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::{FunctionSignature, FunctionVisitor};
use crate::module_translator::ModuleTranslator;
use crate::statement_translator::StatementTranslator;
use crate::value::Value;

pub struct FunctionTranslator<'ctx, 'm> {
    function_signature: FunctionSignature,
    function_ir: FunctionValue<'ctx>,
    arguments_ir: HashMap<String, Value<'ctx>>,
    parent: &'m ModuleTranslator<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionTranslator<'ctx, 'm> {
    type Target = ModuleTranslator<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> FunctionVisitor for FunctionTranslator<'ctx, 'm> {
    fn visit_body(&self, body: &BasicBlock) -> CompilationResult<()> {
        let root_basic_block = self
            .context()
            .append_basic_block(self.function_ir.clone(), "entry");

        self.builder().position_at_end(root_basic_block);

        let statement_translator = StatementTranslator::new(self);
        body.visit(&statement_translator)
    }
}

impl<'ctx, 'm> FunctionTranslator<'ctx, 'm> {
    pub fn new(
        function_ir: FunctionValue<'ctx>,
        function_signature: &FunctionSignature,
        parent: &'m ModuleTranslator<'ctx>,
    ) -> CompilationResult<Self> {
        let builder = parent.context().create_builder();
        let mut arguments_ir = HashMap::with_capacity(function_ir.count_params() as usize);
        for (arg_id, arg) in function_signature.args.iter().enumerate() {
            let arg_ir = function_ir.get_nth_param(arg_id as u32).unwrap();
            let arg_type = &arg.value_type;
            arguments_ir.insert(
                arg.name.clone(),
                Value::from_ir(arg_ir.as_any_value_enum(), arg_type)?,
            );
        }

        Ok(Self {
            function_signature: function_signature.clone(),
            function_ir,
            arguments_ir,
            parent,
            builder,
        })
    }

    #[inline(always)]
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    #[inline(always)]
    pub fn function_signature(&self) -> &FunctionSignature {
        &self.function_signature
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.arguments_ir.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(CompilationError::UnresolvedName(name.to_string())),
        }
    }
}
