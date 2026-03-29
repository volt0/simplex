use crate::expression::Expression;

pub enum Statement {
    Return(Box<Expression>),
}

impl Statement {
    pub fn new_return(expression: Box<Expression>) -> Statement {
        Statement::Return(expression)
    }
}
