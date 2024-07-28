use std::ops::Deref;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::targets::TargetTriple;

use crate::function::Function;

pub struct Module<'ctx> {
    ir: ModuleIr<'ctx>,
}

impl<'ctx> Deref for Module<'ctx> {
    type Target = ModuleIr<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.ir
    }
}

impl<'ctx> Module<'ctx> {
    pub fn new(name: &str, ctx: &'ctx BackendContext) -> Self {
        let ir = ctx.create_module(name);
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));
        Module { ir }
    }

    pub fn compile(&self, defs: Vec<crate::ast::Definition>, ctx: &'ctx BackendContext) {
        for definition in defs.iter() {
            let function = Function::new(definition.name.as_ref(), self, ctx);
            function.compile(vec![], self, ctx);
        }
    }
}
