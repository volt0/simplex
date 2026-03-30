use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::FunctionValue;

use crate::basic_block::BasicBlock;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::{Function, FunctionVisitor};
use crate::statement_translator::StatementTranslator;
use crate::value::Value;

pub struct FunctionTranslator<'ctx> {
    pub function_ir: FunctionValue<'ctx>,
    pub arguments_ir: HashMap<String, Value<'ctx>>,
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
}

impl<'ctx> FunctionVisitor for FunctionTranslator<'ctx> {
    fn visit_body(&self, body: &BasicBlock) -> CompilationResult<()> {
        let root_basic_block = self
            .context()
            .append_basic_block(self.function_ir.clone(), "entry");

        self.builder().position_at_end(root_basic_block);

        let statement_translator = StatementTranslator::new(self);
        statement_translator.translate_basic_block(body)
    }
}

impl<'ctx> FunctionTranslator<'ctx> {
    pub fn new(
        function_ir: FunctionValue<'ctx>,
        function: &Function,
        context: &'ctx Context,
    ) -> CompilationResult<Self> {
        let builder = context.create_builder();
        let mut arguments_ir = HashMap::with_capacity(function_ir.count_params() as usize);
        for (arg_id, arg) in function.signature.args.iter().enumerate() {
            let arg_ir = function_ir.get_nth_param(arg_id as u32).unwrap();
            let arg_type = &arg.value_type;
            arguments_ir.insert(arg.name.clone(), Value::from_ir(arg_ir, arg_type)?);
        }

        Ok(Self {
            function_ir,
            arguments_ir,
            context,
            builder,
        })
    }

    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.arguments_ir.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(CompilationError::UnresolvedName(name.to_string())),
        }
    }
}
