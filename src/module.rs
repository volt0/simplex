use crate::ast;
use crate::definition::Definition;
use crate::function::{compile_function, Function};
use std::rc::Rc;

pub fn compile_module(module_ast: ast::Module) -> ModuleBuilder {
    let mut builder = ModuleBuilder {
        inner: Module {
            definitions: vec![],
        },
    };

    for definition_ast in module_ast.definitions {
        match definition_ast.value {
            ast::DefinitionValue::Function(function_ast) => {
                let function_builder = compile_function(function_ast);
                let function = function_builder.build();
                builder.add_function(function);
            }
        }
    }

    builder
}

pub struct ModuleBuilder {
    inner: Module,
}

impl ModuleBuilder {
    pub fn add_function(&mut self, function: Rc<Function>) {
        self.inner.definitions.push(Definition::Function(function));
    }

    pub fn build(self) -> Rc<Module> {
        let ModuleBuilder { inner: module } = self;

        let module = Rc::new(module);
        let module_visitor = ModulePass {
            module: module.clone(),
        };
        module.traversal(&module_visitor);
        module
    }
}

pub trait ModuleVisitor {
    fn visit_function(&self, function: &Function);
}

pub struct Module {
    definitions: Vec<Definition>,
}

impl Module {
    pub fn traversal(&self, visitor: &dyn ModuleVisitor) {
        for definition in &self.definitions {
            definition.traversal(visitor);
        }
    }
}

struct ModulePass {
    module: Rc<Module>,
}

impl ModuleVisitor for ModulePass {
    fn visit_function(&self, function: &Function) {
        todo!();
    }
}
