use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::targets::TargetTriple;

use crate::definition::Definition;

pub struct Module {
    name: Rc<str>,
    defs: Vec<Definition>,
}

impl Module {
    pub fn new(name: Rc<str>, defs: Vec<Definition>) -> Self {
        Module { name, defs }
    }

    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> ModuleIr<'ctx> {
        let module_ir = ctx.create_module(self.name.as_ref());
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        for definition in self.defs.iter().cloned() {
            definition.compile(&module_ir, ctx);
        }

        module_ir
    }
}
