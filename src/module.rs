use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::targets::TargetTriple;

use crate::definition::{Definition, DefinitionValue};

pub struct Module {
    pub name: Rc<str>,
    pub defs: Vec<Definition>,
}

impl Module {
    pub fn compile<'ctx>(&self, ctx: &'ctx BackendContext) -> ModuleIr<'ctx> {
        let module_ir = ctx.create_module("test");
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        for definition in self.defs.iter().cloned() {
            let name = definition.name.as_ref();
            match definition.value {
                DefinitionValue::Function(function) => {
                    let function = function.as_ref();
                    function.compile(name, &module_ir, ctx);
                }
            }
        }

        module_ir
    }
}
