use std::collections::HashMap;

use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use crate::definition::Definition;

type ModuleIR<'ctx> = inkwell::module::Module<'ctx>;

pub struct Module<'ctx> {
    pub(crate) module_ir: ModuleIR<'ctx>,
    pub defs: HashMap<String, Definition<'ctx>>,
}

impl<'ctx> Module<'ctx> {
    pub fn new(module_ir: ModuleIR<'ctx>) -> Self {
        Self {
            module_ir,
            defs: HashMap::new(),
        }
    }

    pub fn add_definition(&mut self, name: &String, def: Definition<'ctx>) {
        self.defs.insert(name.clone(), def);
    }

    pub fn run_test(&self) {
        self.module_ir.print_to_stderr();

        type TestFunc = unsafe extern "C" fn(u8, i16, i32, bool) -> i64;

        let execution_engine = self
            .module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let test_func: JitFunction<'_, TestFunc> =
                execution_engine.get_function("test").unwrap();

            let x = 1u8;
            let y = 2i16;
            let z = 3i32;
            let w = true;
            dbg!(test_func.call(x, y, z, w));
        }
    }
}
