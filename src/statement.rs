use crate::basic_block::BasicBlock;
use crate::expression::Expression;

pub enum Statement {
    BasicBlock(BasicBlock),
    Return(Box<Expression>),
}

impl Statement {
    pub fn new_basic_block(statements: Vec<Statement>) -> Statement {
        Statement::BasicBlock(BasicBlock::new(statements))
    }

    pub fn new_return(expression: Box<Expression>) -> Statement {
        Statement::Return(expression)
    }
}
