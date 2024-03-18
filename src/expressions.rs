use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

use crate::constant::Constant;
use crate::scope::Scope;
use crate::types::{IntegerType, Type};
use crate::values::Value;

#[allow(unused)]
pub enum Expression {
    Constant(Constant),
    Identifier(Rc<str>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    UnaryOperation(UnaryOperationExpression, Box<Expression>),
    Cast(Box<Expression>, Type),
    Call(Box<Expression>, Vec<Expression>),
    ItemAccess(Box<Expression>, Box<Expression>),
    MemberAccess(Box<Expression>, Rc<str>),
}

impl Expression {
    pub fn _new_int_const(value: i32) -> Box<Self> {
        Box::new(Expression::Constant(Constant::SignedInteger(
            IntegerType::Int,
            value as i64,
        )))
    }

    pub fn _new_add(a: Box<Expression>, b: Box<Expression>) -> Box<Self> {
        Box::new(Expression::BinaryOperation(BinaryOperation::Add, a, b))
    }

    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Value<'ctx> {
        let _ = builder;
        match self {
            Expression::Constant(constant) => constant.compile(ctx),
            Expression::Identifier(identifier) => scope.resolve(identifier.clone()).clone(),
            // Expression::Conditional(_, _, _) => {}
            // Expression::BinaryOperation(_, _, _) => {}
            // Expression::UnaryOperation(_, _) => {}
            // Expression::Cast(_, _) => {}
            // Expression::Call(_, _) => {}
            // Expression::ItemAccess(_, _) => {}
            // Expression::MemberAccess(_, _) => {}
            _ => todo!(),
        }
    }
}

#[allow(unused)]
pub struct UnaryOperationExpression {
    operation: UnaryOperation,
    val: Box<Expression>,
}

#[allow(unused)]
pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
}

#[allow(unused)]
#[derive(Debug)]
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
