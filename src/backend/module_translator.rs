use std::cell::RefCell;

use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module as ModuleIR;
use inkwell::targets::TargetTriple;
use inkwell::values::BasicValueEnum;
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};

use super::function_translator::FunctionTranslator;
use super::type_translator::TypeTranslator;
use crate::function::Function;
use crate::module::ModuleVisitor;

pub struct ModuleTranslator<'ctx> {
    pub context: &'ctx Context,
    module_ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, BasicValueEnum<'ctx>>>,
}

impl<'ctx> ModuleVisitor for ModuleTranslator<'ctx> {
    fn visit_function(&self, function: &Function) {
        let type_translator = TypeTranslator::new(self);
        let function_type = function.get_function_type();
        let function_type_ir = type_translator.translate_function_type(&function_type);
        let function_ir =
            self.module_ir
                .add_function(function.get_mangled_name(), function_type_ir, None);

        let function_translator = FunctionTranslator::new(self, function_ir);
        function.visit(&function_translator);
    }
}

impl<'ctx> ModuleTranslator<'ctx> {
    pub fn new(context: &'ctx Context) -> ModuleTranslator<'ctx> {
        let ir = context.create_module("test_module");
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        ModuleTranslator {
            context,
            module_ir: ir,
            values: Default::default(),
        }
    }

    pub fn store_value(&self, value: BasicValueEnum<'ctx>) -> DefaultKey {
        self.values.borrow_mut().insert(value)
    }

    pub fn load_value(&self, id: DefaultKey) -> Option<BasicValueEnum<'ctx>> {
        self.values.borrow().get(id).cloned()
    }

    pub fn create_execution_engine(&self) -> ExecutionEngine<'ctx> {
        self.module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap()
    }

    pub fn dbg_print(&self) {
        self.module_ir.print_to_stderr();
    }
}

#[cfg(test)]
pub mod tests {
    use inkwell::context::Context;
    use inkwell::execution_engine::{JitFunction, UnsafeFunctionPointer};
    use inkwell::OptimizationLevel;

    use crate::ast;
    use crate::backend::ModuleTranslator;
    use crate::module::Module;

    use super::*;

    pub fn compile_module_test<F>(module_ast: ast::Module, context: &Context) -> JitFunction<'_, F>
    where
        F: UnsafeFunctionPointer,
    {
        let module = Module::new(module_ast);

        let module_translator = ModuleTranslator::new(&context);
        module.visit(&module_translator);

        module_translator.module_ir.print_to_stderr();

        let execution_engine = module_translator
            .module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe { execution_engine.get_function("sum").unwrap() }
    }
}
