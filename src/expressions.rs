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
    BinaryOperation(BinaryOperationExpression),
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
        Box::new(Expression::BinaryOperation(BinaryOperationExpression {
            operation: BinaryOperation::Add,
            a,
            b,
        }))
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
            Expression::BinaryOperation(op) => op.compile(scope, builder, ctx),
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

pub struct BinaryOperationExpression {
    operation: BinaryOperation,
    a: Box<Expression>,
    b: Box<Expression>,
}

impl BinaryOperationExpression {
    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Value<'ctx> {
        match self.operation {
            BinaryOperation::Add => {
                let a = self.a.compile(scope, builder, ctx);
                let b = self.b.compile(scope, builder, ctx);

                match (&a.value_type, &b.value_type) {
                    (Type::SignedInteger(a_type), Type::SignedInteger(b_type)) => {
                        assert!(a_type == b_type);
                        let a_ir = a.ir.into_int_value();
                        let b_ir = b.ir.into_int_value();
                        let result = builder.build_int_add(a_ir, b_ir, "").unwrap();
                        Value {
                            ir: result.into(),
                            value_type: Type::SignedInteger(a_type.clone()),
                        }
                    }
                    _ => todo!(),
                }
            }
            // BinaryOperation::Sub => {}
            // BinaryOperation::Mul => {}
            // BinaryOperation::Div => {}
            // BinaryOperation::Mod => {}
            // BinaryOperation::BitAnd => {}
            // BinaryOperation::BitXor => {}
            // BinaryOperation::BitOr => {}
            // BinaryOperation::ShiftLeft => {}
            // BinaryOperation::ShiftRight => {}
            // BinaryOperation::Eq => {}
            // BinaryOperation::Ne => {}
            // BinaryOperation::Gt => {}
            // BinaryOperation::Ge => {}
            // BinaryOperation::Lt => {}
            // BinaryOperation::Le => {}
            // BinaryOperation::LogicalAnd => {}
            // BinaryOperation::LogicalOr => {}
            _ => todo!(),
        }
    }
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
