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
    #[inline(always)]
    pub fn new_identifier(name: Rc<str>) -> ExpressionRef {
        Box::new(Expression::Identifier(name))
    }

    #[inline(always)]
    pub fn new_constant(constant: Constant) -> ExpressionRef {
        Box::new(Expression::Constant(constant))
    }

    #[inline(always)]
    pub fn new_call() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_item_access() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_member_access() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_prefix_plus() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_prefix_minus() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_bit_not() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_logic_not() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_cast() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_mul() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_div() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_mod() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_add(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpression {
            operation: BinaryOperation::Add,
            a,
            b,
        }))
    }

    #[inline(always)]
    pub fn new_sub(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpression {
            operation: BinaryOperation::Sub,
            a,
            b,
        }))
    }

    #[inline(always)]
    pub fn new_lshift() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_rshift() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_bit_and() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_bit_xor() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_bit_or() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_lt() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_gt() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_le() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_ge() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_eq() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_ne() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_logic_and() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_logic_or() -> ExpressionRef {
        todo!()
    }

    #[inline(always)]
    pub fn new_inline_if() -> ExpressionRef {
        todo!()
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
