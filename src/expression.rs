use crate::constant::Constant;

pub enum Expression {
    LoadConstant(Constant),
    LoadValue(String),
    Conditional(ConditionalExpression),
    UnaryOperation(UnaryOperationExpression),
    BinaryOperation(BinaryOperationExpression),
    Cast(CastExpression),
    Call(CallExpression),
    ItemAccess(ItemAccessExpression),
    MemberAccess(MemberAccessExpression),
}

#[derive(Clone, Debug)]
pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
}

pub struct UnaryOperationExpression {
    pub operation: UnaryOperation,
    pub arg: Box<Expression>,
}

#[derive(Clone, Debug)]
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
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    LogicalAnd,
    LogicalOr,
}

pub struct BinaryOperationExpression {
    pub operation: BinaryOperation,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

pub struct CastExpression {
    pub expression: Box<Expression>,
    pub target_type: Type,
}

pub struct CallExpression {
    pub callee: Box<Expression>,
    pub arguments: Vec<Box<Expression>>,
}

pub struct ItemAccessExpression {
    pub object: Box<Expression>,
    pub index: Box<Expression>,
}

pub struct MemberAccessExpression {
    pub object: Box<Expression>,
    pub member: String,
}

pub struct ConditionalExpression {
    pub condition: Box<Expression>,
    pub then_expr: Box<Expression>,
    pub else_expr: Box<Expression>,
}

impl Expression {}

pub struct Type;
