use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;

use crate::function::Function;

#[derive(Clone)]
pub enum DefinitionValue {
    Function(Rc<Function>),
}

#[derive(Clone)]
pub struct Definition {
    name: Rc<str>,
    value: DefinitionValue,
}

impl Definition {
    pub fn define_function(name: Rc<str>, function: Rc<Function>) -> Self {
        Definition {
            name,
            value: DefinitionValue::Function(function),
        }
    }

    pub fn compile<'ctx>(&self, module_ir: &ModuleIr<'ctx>, ctx: &'ctx BackendContext) {
        let name = self.name.as_ref();
        match &self.value {
            DefinitionValue::Function(function) => {
                let function = function.as_ref();
                function.compile(name, &module_ir, ctx);
            }
        }
    }
}
