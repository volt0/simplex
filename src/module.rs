use crate::definition::Definition;
use crate::errors::CompilationResult;
use crate::function::Function;

pub trait ModuleVisitor {
    fn visit_function(&mut self, name: Option<&str>, function: &Function) -> CompilationResult<()>;
}

pub struct Module {
    pub defs: Vec<Definition>,
}

impl Module {
    pub fn new(defs: Vec<Definition>) -> Module {
        Module { defs }
    }

    pub fn visit(&self, visitor: &mut dyn ModuleVisitor) -> CompilationResult<()> {
        for def in &self.defs {
            def.visit(visitor)?;
        }
        Ok(())
    }
}
