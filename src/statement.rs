use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::expression::Expression;

pub trait StatementVisitor {
    fn visit_basic_block(&self, block: &BasicBlock) -> CompilationResult<()>;
    fn add_return_statement(&self, expr: &Expression) -> CompilationResult<()>;
}

pub enum Statement {
    BasicBlock(BasicBlock),
    Return(Box<Expression>),
}

impl Statement {
    pub fn new_basic_block(block: BasicBlock) -> Statement {
        Statement::BasicBlock(block)
    }

    pub fn new_return(expr: Box<Expression>) -> Statement {
        Statement::Return(expr)
    }

    pub fn visit(&self, visitor: &dyn StatementVisitor) -> CompilationResult<()> {
        match self {
            Statement::BasicBlock(block) => visitor.visit_basic_block(block),
            Statement::Return(expr) => visitor.add_return_statement(expr),
        }
    }
}
