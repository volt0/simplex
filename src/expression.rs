use crate::constant::Constant;

pub enum Expression {
    LoadConstant(Constant),
    LoadValue(String),
    BinaryOperation(BinaryOperationExpression),
    UnaryOperation(UnaryOperationExpression),
}

#[derive(Copy, Clone)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
    ShiftLeft,
    ShiftRight,
}

pub struct BinaryOperationExpression {
    pub operation: BinaryOperation,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Copy, Clone)]
pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
}

pub struct UnaryOperationExpression {
    pub operation: UnaryOperation,
    pub arg: Box<Expression>,
}

impl Expression {
    pub fn new_load_constant(value: Constant) -> Box<Self> {
        Box::new(Expression::LoadConstant(value))
    }

    pub fn new_load_value(name: String) -> Box<Self> {
        Box::new(Expression::LoadValue(name))
    }

    pub fn new_add(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Add, lhs, rhs)
    }

    pub fn new_sub(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Sub, lhs, rhs)
    }

    pub fn new_mul(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Mul, lhs, rhs)
    }

    pub fn new_div(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Div, lhs, rhs)
    }

    pub fn new_mod(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Mod, lhs, rhs)
    }

    pub fn new_bit_and(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::BitAnd, lhs, rhs)
    }

    pub fn new_bit_xor(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::BitXor, lhs, rhs)
    }

    pub fn new_bit_or(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::BitOr, lhs, rhs)
    }

    pub fn new_shift_left(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::ShiftLeft, lhs, rhs)
    }

    pub fn new_shift_right(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::ShiftRight, lhs, rhs)
    }

    fn new_binary_operation(
        operation: BinaryOperation,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    ) -> Box<Self> {
        Box::new(Expression::BinaryOperation(BinaryOperationExpression {
            operation,
            lhs,
            rhs,
        }))
    }

    pub fn new_unary_plus(arg: Box<Expression>) -> Box<Self> {
        Self::new_unary_operation(UnaryOperation::Plus, arg)
    }

    pub fn new_unary_minus(arg: Box<Expression>) -> Box<Self> {
        Self::new_unary_operation(UnaryOperation::Minus, arg)
    }

    pub fn new_bit_not(arg: Box<Expression>) -> Box<Self> {
        Self::new_unary_operation(UnaryOperation::BitNot, arg)
    }

    fn new_unary_operation(operation: UnaryOperation, arg: Box<Expression>) -> Box<Self> {
        Box::new(Expression::UnaryOperation(UnaryOperationExpression {
            operation,
            arg,
        }))
    }
}
