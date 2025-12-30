use std::ops::Deref;

use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue};

use super::module_translator::ModuleTranslator;
use super::statement_translator::StatementTranslator;
use crate::basic_block::BasicBlock;
use crate::function::FunctionVisitor;
use crate::function_argument::FunctionArgument;

pub struct FunctionTranslator<'ctx, 'm> {
    pub function_ir: FunctionValue<'ctx>,
    pub builder: Builder<'ctx>,
    parent: &'m ModuleTranslator<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionTranslator<'ctx, 'm> {
    type Target = ModuleTranslator<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> FunctionVisitor for FunctionTranslator<'ctx, 'm> {
    fn visit_basic_block(&self, basic_block: &BasicBlock) {
        let basic_block_ir = self.context.append_basic_block(self.function_ir, "");
        self.builder.position_at_end(basic_block_ir);

        let translator = StatementTranslator::new(self);
        basic_block.visit(&translator);
    }
}

impl<'ctx, 'm> FunctionTranslator<'ctx, 'm> {
    pub fn new(parent: &'m ModuleTranslator<'ctx>, ir: FunctionValue<'ctx>) -> Self {
        let context = parent.context;
        let builder = context.create_builder();
        FunctionTranslator {
            parent,
            builder,
            function_ir: ir,
        }
    }

    pub fn load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.function_ir.get_nth_param(arg.id).unwrap()
    }
}
