use std::cell::RefCell;
use std::rc::Rc;

use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::module::Module as ModuleIR;
use inkwell::targets::TargetTriple;
use inkwell::values::BasicValueEnum;
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};

use crate::ast;
use crate::definitions::DefinitionBuilder;
use crate::function::{Function, FunctionTranslator};
use crate::namespace::Namespace;
use crate::types::TypeTranslator;

pub trait ModuleVisitor {
    fn visit_function(&self, function: &Function);
}

pub struct Module {
    body: RefCell<Namespace>,
}

impl Module {
    pub fn visit(&self, visitor: &dyn ModuleVisitor) {
        self.body.borrow().visit(visitor);
    }
}

impl Module {
    fn new() -> Module {
        Module {
            body: RefCell::new(Namespace::default()),
        }
    }

    fn add_function(&self, function: &Rc<Function>) {
        self.body.borrow_mut().add_function(function);
    }
}

pub struct ModuleBuilder {
    module: Rc<Module>,
    definition_builders: Vec<DefinitionBuilder>,
}

impl ModuleBuilder {
    pub fn from_ast(module_ast: ast::Module) -> Self {
        let module = Rc::new(Module::new());
        let mut builder = ModuleBuilder {
            module,
            definition_builders: vec![],
        };

        for definition_ast in module_ast.definitions {
            builder.create_definition(definition_ast);
        }

        builder
    }

    pub fn create_definition(&mut self, definition_ast: ast::Definition) {
        match definition_ast.value {
            ast::DefinitionValue::Function(function_ast) => {
                self.create_function(definition_ast.name, function_ast);
            }
        }
    }

    pub fn build(self) -> Rc<Module> {
        for builder in self.definition_builders.into_iter() {
            builder.build();
        }

        self.module
    }
}

impl ModuleBuilder {
    fn create_function(&mut self, name: String, function_ast: ast::Function) {
        let function = Function::from_ast(&function_ast, Some(name));
        self.module.add_function(&function);

        let builder = DefinitionBuilder::define_function(&function, function_ast);
        self.definition_builders.push(builder);
    }
}

pub struct ModuleTranslator<'ctx> {
    pub context: &'ctx Context,
    module_ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, BasicValueEnum<'ctx>>>,
}

impl<'ctx> ModuleVisitor for ModuleTranslator<'ctx> {
    fn visit_function(&self, function: &Function) {
        let type_translator = TypeTranslator::new(self);
        let function_type = function.function_type();
        let function_type_ir = type_translator.translate_function_type(&function_type);
        let function_ir =
            self.module_ir
                .add_function(function.mangled_name(), function_type_ir, None);

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

    use super::*;

    pub fn compile_module_test<F>(module_ast: ast::Module, context: &Context) -> JitFunction<'_, F>
    where
        F: UnsafeFunctionPointer,
    {
        let module_builder = ModuleBuilder::from_ast(module_ast);
        let module = module_builder.build();

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
