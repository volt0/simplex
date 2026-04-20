use crate::ast::Expression;
use crate::errors::CompilationResult;
use crate::statement::Statement;

pub trait BlockVisitor {
    fn enter_block(&self, block: &Block) -> CompilationResult<()>;
    fn add_return_statement(&self, expr: &Expression) -> CompilationResult<()>;
}

pub struct Block {
    statements: Vec<Statement>,
}

impl Block {
    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn visit(&self, visitor: &dyn BlockVisitor) -> CompilationResult<()> {
        for stmt in self.statements.iter() {
            match stmt {
                Statement::Block(block) => visitor.enter_block(block)?,
                Statement::Return(expr) => visitor.add_return_statement(expr)?,
            }
        }
        Ok(())
    }
}
