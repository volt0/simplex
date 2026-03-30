use crate::errors::CompilationResult;
use crate::function::Function;

pub trait ModuleVisitor {
    fn visit_function(&self, function: &Function) -> CompilationResult<()>;
}

pub struct Module {
    pub definitions: Vec<Definition>,
}

pub enum Definition {
    Function(Function),
}

impl Module {
    pub fn new(definitions: Vec<Definition>) -> Module {
        Module { definitions }
    }

    pub fn visit(&self, visitor: &dyn ModuleVisitor) -> CompilationResult<()> {
        for definition in &self.definitions {
            match definition {
                Definition::Function(function) => visitor.visit_function(function)?,
            }
        }
        Ok(())
    }
}
