#![allow(unused)]

use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::AnyValueEnum;

use crate::types::{IntegerType, Type};
use crate::values::{Constant, Identifier};

pub enum Expression {
    Constant(Constant),
    Identifier(Identifier),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    BinaryOperation(BinaryOperation, Box<Expression>, Box<Expression>),
    UnaryOperation(UnaryOperationExpression, Box<Expression>),
    Cast(Box<Expression>, Type),
    Call(Box<Expression>, Vec<Expression>),
    ItemAccess(Box<Expression>, Box<Expression>),
    MemberAccess(Box<Expression>, Rc<str>),
}

impl Expression {
    pub fn new_int_const(value: i32) -> Box<Self> {
        Box::new(Expression::Constant(Constant::SignedInteger(
            IntegerType::Int,
            value as i64,
        )))
    }

    pub fn new_add(a: Box<Expression>, b: Box<Expression>) -> Box<Self> {
        Box::new(Expression::BinaryOperation(BinaryOperation::Add, a, b))
    }

    pub fn compile<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> AnyValueEnum<'ctx> {
        match self {
            Expression::Constant(constant) => constant.compile(builder, ctx),
            // Expression::Identifier(_) => {}
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

pub struct UnaryOperationExpression {
    operation: UnaryOperation,
    val: Box<Expression>,
}

pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
}

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
