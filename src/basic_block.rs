use crate::errors::CompilationResult;
use crate::statement::Statement;

pub trait BasicBlockVisitor {
    fn visit_statement(&self, statement: &Statement) -> CompilationResult<()>;
}

pub struct BasicBlock {
    statements: Vec<Statement>,
}

impl BasicBlock {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn visit(&self, visitor: &dyn BasicBlockVisitor) -> CompilationResult<()> {
        for stmt in self.statements.iter() {
            visitor.visit_statement(stmt)?;
        }
        Ok(())
    }
}
