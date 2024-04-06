use crate::expressions::ExpressionRef;

#[allow(unused)]
pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
}

#[allow(unused)]
pub struct UnaryOperationExpression {
    operation: UnaryOperation,
    val: ExpressionRef,
}
