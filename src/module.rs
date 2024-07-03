use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::targets::TargetTriple;

use crate::definition::Definition;
use crate::scope::Scope;
use crate::values::Value;

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

        let scope = ModuleScope {};
        for definition in self.defs.iter().cloned() {
            definition.compile(&scope, &module_ir, ctx);
        }

        module_ir
    }
}

pub struct ModuleScope {}

impl<'ctx> Scope<'ctx> for ModuleScope {
    fn resolve(&self, name: Rc<str>) -> &Value<'ctx> {
        panic!("Undefined: {}", name.as_ref())
    }
}
