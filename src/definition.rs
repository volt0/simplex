use crate::function::Function;
use crate::module::ModuleVisitor;
use std::rc::Rc;

pub enum Definition {
    Function(Rc<Function>),
}

impl Definition {
    pub fn traversal_pass(&self) {
        match self {
            Definition::Function(function) => function.traversal_pass(),
        }
    }

    pub fn visit(&self, visitor: &dyn ModuleVisitor) {
        match self {
            Definition::Function(function) => visitor.visit_function(&function),
        }
    }
}
