use crate::block::Block;
use crate::errors::CompilationResult;
use crate::expression::Expression;

pub trait StatementVisitor {
    fn visit_block(&self, block: &Block) -> CompilationResult<()>;
    fn add_return_statement(&self, expr: &Expression) -> CompilationResult<()>;
}

pub enum Statement {
    Block(Block),
    Return(Box<Expression>),
}

impl Statement {
    pub fn new_block(block: Block) -> Statement {
        Statement::Block(block)
    }

    pub fn new_return(expr: Box<Expression>) -> Statement {
        Statement::Return(expr)
    }

    pub fn visit(&self, visitor: &dyn StatementVisitor) -> CompilationResult<()> {
        match self {
            Statement::Block(block) => visitor.visit_block(block),
            Statement::Return(expr) => visitor.add_return_statement(expr),
        }
    }
}
