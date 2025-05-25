use crate::ast;
use crate::definition::Definition;
use crate::function::{Function, FunctionBuilder};
use std::rc::Rc;

pub struct ModuleBuilder {
    inner: Module,
    pending_functions: Vec<(Rc<Function>, ast::BasicBlock)>,
}

impl ModuleBuilder {
    pub fn from_ast(module_ast: ast::Module) -> Self {
        let mut builder = ModuleBuilder {
            inner: Module {
                definitions: vec![],
            },
            pending_functions: vec![],
        };

        for definition_ast in module_ast.definitions {
            match definition_ast.value {
                ast::DefinitionValue::Function(function_ast) => {
                    let function_builder = FunctionBuilder::from_ast(function_ast.signature);
                    let function = function_builder.build();
                    builder.add_function(&function);

                    if let Some(body) = function_ast.payload {
                        builder.pending_functions.push((function, body));
                    }
                }
            }
        }

        builder
    }

    pub fn add_function(&mut self, function: &Rc<Function>) {
        self.inner
            .definitions
            .push(Definition::Function(function.clone()));
    }

    pub fn build(self) -> Rc<Module> {
        let ModuleBuilder {
            inner: module,
            pending_functions,
        } = self;

        for (function_impl_builder, payload) in pending_functions {
            function_impl_builder.init_implementation(payload);
        }

        Rc::new(module)
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
