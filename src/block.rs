use crate::errors::CompilationResult;
use crate::statement::{Statement, StatementVisitor};

pub struct Block {
    statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn visit(&self, visitor: &dyn StatementVisitor) -> CompilationResult<()> {
        for stmt in self.statements.iter() {
            stmt.visit(visitor)?;
        }
        Ok(())
    }
}
