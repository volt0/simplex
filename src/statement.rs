use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::expression::Expression;

pub trait StatementVisitor {
    fn visit_basic_block(&self, basic_block: &BasicBlock) -> CompilationResult<()>;
    fn add_return_statement(&self, expression: &Expression) -> CompilationResult<()>;
}

pub enum Statement {
    BasicBlock(BasicBlock),
    Return(Box<Expression>),
}

impl Statement {
    pub fn new_basic_block(basic_block: BasicBlock) -> Statement {
        Statement::BasicBlock(basic_block)
    }

    pub fn new_return(expression: Box<Expression>) -> Statement {
        Statement::Return(expression)
    }

    pub fn visit(&self, visitor: &dyn StatementVisitor) -> CompilationResult<()> {
        match self {
            Statement::BasicBlock(basic_block) => visitor.visit_basic_block(basic_block),
            Statement::Return(expression) => visitor.add_return_statement(expression),
        }
    }
}
