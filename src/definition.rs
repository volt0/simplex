use crate::function::Function;
use crate::module::ModuleVisitor;
use std::rc::Rc;

pub enum Definition {
    Function(Rc<Function>),
}

impl Definition {
    pub fn traversal(&self, visitor: &dyn ModuleVisitor) {
        match self {
            Definition::Function(function) => visitor.visit_function(&function),
        }
    }
}
