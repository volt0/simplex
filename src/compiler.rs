use crate::scope::Scope;
use crate::value::Identifier;
use inkwell::context::Context as BackendContext;
use std::ops::Deref;

pub struct Compiler<'ctx> {
    ctx: &'ctx BackendContext,
}

impl<'ctx> Deref for Compiler<'ctx> {
    type Target = BackendContext;

    fn deref(&self) -> &Self::Target {
        &self.ctx
    }
}

impl<'ctx> Scope<'ctx> for Compiler<'ctx> {
    fn lookup(&self, _: &str) -> Option<Identifier<'ctx>> {
        None
    }
}

impl<'ctx> Compiler<'ctx> {
    pub fn new(backend_ctx: &'ctx BackendContext) -> Self {
        Compiler { ctx: backend_ctx }
    }

    pub fn context(&self) -> &'ctx BackendContext {
        &self.ctx
    }
}
