use crate::ast;
use crate::function::Function;
use std::cell::RefCell;
use std::rc::Rc;

pub trait ModuleVisitor {
    fn visit_function(&self, function: &Function);
}

pub struct Module {
    inner: RefCell<ModuleInner>,
}

impl Module {
    pub fn new() -> Rc<Self> {
        Rc::new(Module {
            inner: RefCell::default(),
        })
    }

    pub fn from_ast(module_ast: &ast::Module) -> Rc<Self> {
        let module = Self::new();

        let mut queue = vec![];
        for def in module_ast.definitions.iter() {
            match &def.value {
                ast::DefinitionValue::Function(function_ast) => {
                    let function = Function::from_ast(&function_ast.signature, module.clone());
                    module.add_function(function.clone());

                    if let Some(entry_basic_block) = &function_ast.payload {
                        queue.push((entry_basic_block, function));
                    }
                }
            }
        }

        for (entry_basic_block, function) in queue {
            function.init_implementation(entry_basic_block);
        }

        module
    }

    pub fn add_function(&self, function: Rc<Function>) {
        let mut inner = self.inner.borrow_mut();
        inner.functions.push(function);
    }

    pub fn traversal(&self, visitor: &dyn ModuleVisitor) {
        let inner = self.inner.borrow();
        for function in inner.functions.iter().cloned() {
            visitor.visit_function(&function);
        }
    }
}

#[derive(Default)]
pub struct ModuleInner {
    functions: Vec<Rc<Function>>,
}
