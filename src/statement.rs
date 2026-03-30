use crate::basic_block::BasicBlock;
use crate::expression::Expression;

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
}
