use inkwell::context::Context;

use crate::module::Module;
use crate::module_translator::ModuleTranslator;

pub struct Translator {
    context: Context,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            context: Context::create(),
        }
    }

    #[inline(always)]
    pub fn context(&self) -> &Context {
        &self.context
    }

    pub fn translate_module(&self, module: &Module) {
        let mut module_translator = ModuleTranslator::new(self);
        module.visit(&mut module_translator).unwrap();
        module_translator.run_test();
    }
}
