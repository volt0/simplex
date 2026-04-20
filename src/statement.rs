use crate::block::Block;
use crate::expression::Expression;

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
}
