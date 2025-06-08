use super::function_compiler::FunctionCompiler;
use super::type_compiler::TypeCompiler;
use crate::function::Function;
use crate::module::ModuleVisitor;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module as ModuleIR;
use inkwell::targets::TargetTriple;
use inkwell::values::BasicValueEnum;
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};
use std::cell::RefCell;

pub struct ModuleCompiler<'ctx> {
    context: &'ctx Context,
    ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, BasicValueEnum<'ctx>>>,
}

impl<'ctx> ModuleVisitor for ModuleCompiler<'ctx> {
    fn visit_function(&self, function: &Function) {
        let type_compiler = TypeCompiler::new(self);
        let function_type = function.function_type();
        let function_type_ir = type_compiler.compile_function_type(&function_type);
        let function_ir = self.ir.add_function("sum", function_type_ir, None);
        let function_compiler = FunctionCompiler::new(self, function_ir);
        function.visit(&function_compiler);
    }
}

impl<'ctx> ModuleCompiler<'ctx> {
    pub fn new(context: &'ctx Context) -> ModuleCompiler<'ctx> {
        let ir = context.create_module("test_module");
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        ModuleCompiler {
            context,
            ir,
            values: Default::default(),
        }
    }

    #[inline(always)]
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn store_value(&self, value: BasicValueEnum<'ctx>) -> DefaultKey {
        self.values.borrow_mut().insert(value)
    }

    pub fn load_value(&self, id: DefaultKey) -> Option<BasicValueEnum<'ctx>> {
        self.values.borrow().get(id).cloned()
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast;
    use crate::module::Module;
    use inkwell::execution_engine::UnsafeFunctionPointer;

    pub fn compile_module_test<F>(module_ast: ast::Module, context: &Context) -> JitFunction<F>
    where
        F: UnsafeFunctionPointer,
    {
        let module = Module::from_ast(module_ast);
        module.traversal_pass();

        let module_compiler = ModuleCompiler::new(&context);
        module.visit(&module_compiler);

        module_compiler.ir.print_to_stderr();

        let execution_engine = module_compiler
            .ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe { execution_engine.get_function("sum").unwrap() }
    }
}
