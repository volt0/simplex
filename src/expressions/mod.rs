use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

use binary_operation::BinaryOperationExpr;
pub use constant::Constant;
use unary_operation::UnaryOperationExpression;

use crate::scope::Scope;
use crate::types::TypeSpec;
use crate::values::Value;

mod binary_operation;
mod constant;
mod unary_operation;

pub type ExpressionRef = Box<Expression>;

#[allow(unused)]
pub enum Expression {
    Constant(Constant),
    Identifier(Rc<str>),
    Conditional(ConditionalExpression),
    BinaryOperation(BinaryOperationExpr),
    UnaryOperation(UnaryOperationExpression),
    Cast(CastExpression),
    Call(CallExpression),
    ItemAccess(ItemAccessExpression),
    MemberAccess(MemberAccessExpression),
}

impl Expression {
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
    pub fn new_add(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_add(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_sub(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_sub(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_mul(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_mul(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_div(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_div(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_mod(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_mod(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_bit_and(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(
            BinaryOperationExpr::new_bit_and(a, b),
        ))
    }

    #[inline(always)]
    pub fn new_bit_xor(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(
            BinaryOperationExpr::new_bit_xor(a, b),
        ))
    }

    #[inline(always)]
    pub fn new_bit_or(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(
            BinaryOperationExpr::new_bit_or(a, b),
        ))
    }

    #[inline(always)]
    pub fn new_shift_left(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(
            BinaryOperationExpr::new_shift_left(a, b),
        ))
    }

    #[inline(always)]
    pub fn new_shift_right(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(
            BinaryOperationExpr::new_shift_right(a, b),
        ))
    }

    #[inline(always)]
    pub fn new_lt(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_lt(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_gt(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_gt(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_le(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_le(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_ge(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_ge(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_eq(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_eq(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_ne(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(BinaryOperationExpr::new_ne(
            a, b,
        )))
    }

    #[inline(always)]
    pub fn new_logic_and(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(
            BinaryOperationExpr::new_logic_and(a, b),
        ))
    }

    #[inline(always)]
    pub fn new_logic_or(a: ExpressionRef, b: ExpressionRef) -> ExpressionRef {
        Box::new(Expression::BinaryOperation(
            BinaryOperationExpr::new_logic_or(a, b),
        ))
    }

    #[inline(always)]
    pub fn new_inline_if() -> ExpressionRef {
        todo!()
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
