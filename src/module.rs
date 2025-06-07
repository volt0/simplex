use crate::ast;
use crate::definition::Definition;
use crate::function::Function;
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
