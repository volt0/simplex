use crate::ast;
use crate::definition::Definition;
use crate::function::{Function, FunctionCompiler};
use crate::types::TypeCompiler;

use inkwell::context::Context;
use inkwell::module::Module as ModuleIR;
use inkwell::targets::TargetTriple;
use inkwell::values::BasicValueEnum;
use slotmap::{DefaultKey, SlotMap};
use std::cell::RefCell;
use std::rc::Rc;

pub trait ModuleVisitor {
    fn visit_function(&self, function: &Function);
}

struct ModuleInner {
    definitions: Vec<Definition>,
}

pub struct Module {
    inner: RefCell<ModuleInner>,
}

impl Module {
    pub fn from_ast(module_ast: ast::Module) -> Rc<Self> {
        let module = Rc::new(Module {
            inner: RefCell::new(ModuleInner {
                definitions: vec![],
            }),
        });

        for definition_ast in module_ast.definitions {
            match definition_ast.value {
                ast::DefinitionValue::Function(function_ast) => {
                    module.create_function(function_ast);
                }
            }
        }

        module
    }

    pub fn create_function(&self, function_ast: ast::Function) {
        let function = Function::from_ast(function_ast);
        let mut inner = self.inner.borrow_mut();
        inner.definitions.push(Definition::Function(function));
    }

    pub fn traversal_pass(&self) {
        let inner = self.inner.borrow();
        for definition in &inner.definitions {
            definition.traversal_pass();
        }
    }

    pub fn visit(&self, visitor: &dyn ModuleVisitor) {
        let inner = self.inner.borrow();
        for definition in &inner.definitions {
            definition.visit(visitor);
        }
    }
}

pub struct ModuleCompiler<'ctx> {
    pub context: &'ctx Context,
    pub module_ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, BasicValueEnum<'ctx>>>,
}

impl<'ctx> ModuleVisitor for ModuleCompiler<'ctx> {
    fn visit_function(&self, function: &Function) {
        let type_compiler = TypeCompiler::new(self);
        let function_type = function.function_type();
        let function_type_ir = type_compiler.compile_function_type(&function_type);
        let function_ir = self.module_ir.add_function("sum", function_type_ir, None);
        let function_compiler = FunctionCompiler::new(self, function_ir);
        function.visit(&function_compiler);
    }
}

impl<'ctx> ModuleCompiler<'ctx> {
    pub fn new(context: &'ctx Context) -> ModuleCompiler<'ctx> {
        let ir = context.create_module("test_module");
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        ModuleCompiler {
            context: context,
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
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use crate::ast;
    use crate::module::{Module, ModuleCompiler};
    use inkwell::execution_engine::{JitFunction, UnsafeFunctionPointer};
    use inkwell::OptimizationLevel;

    pub fn compile_module_test<F>(module_ast: ast::Module, context: &Context) -> JitFunction<F>
    where
        F: UnsafeFunctionPointer,
    {
        let module = Module::from_ast(module_ast);
        module.traversal_pass();

        let module_compiler = ModuleCompiler::new(&context);
        module.visit(&module_compiler);

        module_compiler.module_ir.print_to_stderr();

        let execution_engine = module_compiler
            .module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe { execution_engine.get_function("sum").unwrap() }
    }
}
