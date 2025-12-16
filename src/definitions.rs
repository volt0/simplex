use std::rc::Rc;

use crate::ast;
use crate::function::{Function, FunctionBuilder};
use crate::module::ModuleVisitor;

pub enum Definition {
    Function(Rc<Function>),
}

impl Definition {
    pub fn visit(&self, visitor: &dyn ModuleVisitor) {
        match self {
            Definition::Function(function) => visitor.visit_function(&function),
        }
    }
}

pub enum DefinitionBuilder {
    Function(FunctionBuilder),
}

impl DefinitionBuilder {
    pub fn define_function(function: &Rc<Function>, function_ast: ast::Function) -> Self {
        DefinitionBuilder::Function(FunctionBuilder::new(function, function_ast))
    }

    pub fn build(self) -> Rc<Definition> {
        Rc::new(match self {
            DefinitionBuilder::Function(builder) => Definition::Function(builder.build()),
        })
    }
}
