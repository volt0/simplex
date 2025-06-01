use super::basic_block_compiler::BasicBlockCompiler;
use super::module_compiler::ModuleCompiler;
use crate::basic_block::BasicBlock;
use crate::function::{FunctionArgument, FunctionBody, FunctionVisitor};
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::ops::Deref;

pub struct FunctionCompiler<'ctx, 'm> {
    module_compiler: &'m ModuleCompiler<'ctx>,
    ir: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionCompiler<'ctx, 'm> {
    type Target = ModuleCompiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.module_compiler
    }
}

impl<'ctx, 'm> FunctionVisitor for FunctionCompiler<'ctx, 'm> {
    fn visit_body(&self, body: &FunctionBody) {
        match body {
            FunctionBody::BasicBlock(root_basic_block) => self.visit_basic_block(root_basic_block),
            _ => todo!(),
        }
    }

    fn visit_basic_block(&self, basic_block: &BasicBlock) {
        let basic_block_ir = self.context().append_basic_block(self.ir, "");
        self.builder.position_at_end(basic_block_ir);

        let basic_block_compiler = BasicBlockCompiler::new(self);
        basic_block.traversal(&basic_block_compiler);
    }
}

impl<'ctx, 'm> FunctionCompiler<'ctx, 'm> {
    pub fn new(module_compiler: &'m ModuleCompiler<'ctx>, ir: FunctionValue<'ctx>) -> Self {
        let context = module_compiler.context();
        let builder = context.create_builder();
        FunctionCompiler {
            module_compiler,
            ir,
            builder,
        }
    }

    #[inline(always)]
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.ir.get_nth_param(arg.pos_id).unwrap()
    }
}
