use std::rc::Rc;

use crate::definitions::Definition;
use crate::function::Function;
use crate::module::ModuleVisitor;

#[derive(Default)]
pub struct Namespace {
    definitions: Vec<Definition>,
}

impl Namespace {
    pub fn visit(&self, visitor: &dyn ModuleVisitor) {
        for definition in self.definitions.iter() {
            definition.visit(visitor);
        }
    }

    pub fn add_function(&mut self, function: &Rc<Function>) {
        self.add_definition(Definition::Function(function.clone()));
    }

    fn add_definition(&mut self, definition: Definition) {
        self.definitions.push(definition);
    }
}
