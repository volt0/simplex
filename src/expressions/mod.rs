use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

use binary_operation::{BinaryOperation, BinaryOperationExpression};
pub use constant::Constant;
use unary_operation::UnaryOperationExpression;
pub use value::Value;

use crate::scope::Scope;
use crate::types::TypeSpec;

mod binary_operation;
mod constant;
mod unary_operation;
mod value;

pub type ExpressionRef = Box<Expression>;

#[allow(unused)]
pub enum Expression {
    Constant(Constant),
    Identifier(Rc<str>),
    Conditional(ConditionalExpression),
    BinaryOperation(BinaryOperationExpression),
    UnaryOperation(UnaryOperationExpression),
    Cast(CastExpression),
    Call(CallExpression),
    ItemAccess(ItemAccessExpression),
    MemberAccess(MemberAccessExpression),
}

impl Expression {
    // pub fn new_const(Constant) -> ExpressionRef {
    //     Box::new(Expression::Constant(Constant::SignedInteger(
    //         IntegerType::Int,
    //         value as i64,
    //     )))
    // }

    pub fn new_add(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
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
            Expression::BinaryOperation(op) => op.compile(scope, builder, ctx).unwrap(),
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
pub struct ConditionalExpression(ExpressionRef, ExpressionRef, ExpressionRef);

#[allow(unused)]
pub struct CastExpression(ExpressionRef, TypeSpec);

#[allow(unused)]
pub struct CallExpression(ExpressionRef, Vec<Expression>);

#[allow(unused)]
pub struct ItemAccessExpression(ExpressionRef, ExpressionRef);

#[allow(unused)]
pub struct MemberAccessExpression(ExpressionRef, Rc<str>);
