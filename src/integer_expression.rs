use crate::integer_type::IntegerType;

pub struct IntegerExpression {
    exp_type: IntegerType,
    instruction: IntegerInstruction,
}

impl IntegerExpression {
    // pub fn from_ast(exp_ast: &ast::Expression, type_hint: &TypeHint, scope: &dyn LocalScope) -> Box<Self> {
    //     todo!()
    // }
}

pub enum IntegerInstruction {
    Plus(Box<IntegerExpression>),
    Minus(Box<IntegerExpression>),
    BitNot(Box<IntegerExpression>),
    LogicalNot(Box<IntegerExpression>),
    Add(Box<IntegerExpression>, Box<IntegerExpression>),
    Sub(Box<IntegerExpression>, Box<IntegerExpression>),
    Mul(Box<IntegerExpression>, Box<IntegerExpression>),
    Div(Box<IntegerExpression>, Box<IntegerExpression>),
    Mod(Box<IntegerExpression>, Box<IntegerExpression>),
    BitAnd(Box<IntegerExpression>, Box<IntegerExpression>),
    BitXor(Box<IntegerExpression>, Box<IntegerExpression>),
    BitOr(Box<IntegerExpression>, Box<IntegerExpression>),
    ShiftLeft(Box<IntegerExpression>, Box<IntegerExpression>),
    ShiftRight(Box<IntegerExpression>, Box<IntegerExpression>),
    Eq(Box<IntegerExpression>, Box<IntegerExpression>),
    Ne(Box<IntegerExpression>, Box<IntegerExpression>),
    Gt(Box<IntegerExpression>, Box<IntegerExpression>),
    Ge(Box<IntegerExpression>, Box<IntegerExpression>),
    Lt(Box<IntegerExpression>, Box<IntegerExpression>),
    Le(Box<IntegerExpression>, Box<IntegerExpression>),
    LogicalAnd(Box<IntegerExpression>, Box<IntegerExpression>),
    LogicalOr(Box<IntegerExpression>, Box<IntegerExpression>),
}
