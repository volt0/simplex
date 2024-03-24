use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::BasicValueEnum;

use crate::constant::Constant;
use crate::errors::CompilationError;
use crate::scope::Scope;
use crate::types::TypeSpec;
use crate::values::Value;

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
pub struct UnaryOperationExpression {
    operation: UnaryOperation,
    val: ExpressionRef,
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
    a: ExpressionRef,
    b: ExpressionRef,
}

impl BinaryOperationExpression {
    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Result<Value<'ctx>, CompilationError> {
        let a = self.a.compile(scope, builder, ctx);
        let b = self.b.compile(scope, builder, ctx);
        assert_eq!(a.get_type(), b.get_type());

        match a.ir {
            // BasicValueEnum::ArrayValue(_) => {}
            BasicValueEnum::IntValue(_) => {
                let a_ir = a.ir.into_int_value();
                let b_ir = b.ir.into_int_value();

                let result = match self.operation {
                    BinaryOperation::Add => builder.build_int_add(a_ir, b_ir, "")?,
                    BinaryOperation::Sub => builder.build_int_sub(a_ir, b_ir, "")?,
                    BinaryOperation::Mul => builder.build_int_mul(a_ir, b_ir, "")?,
                    // BinaryOperation::Div => builder.build_int_unsigned_div(a_ir, b_ir, "")?,
                    // BinaryOperation::Mod => builder.build_int_unsigned_rem(a_ir, b_ir, "")?,
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
                };

                Ok(Value::from_ir(result.into()))
            }
            // BasicValueEnum::FloatValue(_) => {}
            // BasicValueEnum::FunctionValue(_) => {}
            // BasicValueEnum::PointerValue(_) => {}
            // BasicValueEnum::StructValue(_) => {}
            // BasicValueEnum::VectorValue(_) => {}
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
