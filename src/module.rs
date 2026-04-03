use crate::definition::Definition;
use crate::errors::CompilationResult;
use crate::function::Function;

pub trait ModuleVisitor {
    fn visit_function(&mut self, name: Option<&str>, function: &Function) -> CompilationResult<()>;
}

pub struct Module {
    pub definitions: Vec<Definition>,
}

impl Module {
    pub fn new(definitions: Vec<Definition>) -> Module {
        Module { definitions }
    }

    pub fn visit(&self, visitor: &mut dyn ModuleVisitor) -> CompilationResult<()> {
        for definition in &self.definitions {
            definition.visit(visitor)?;
        }
        Ok(())
    }
}
