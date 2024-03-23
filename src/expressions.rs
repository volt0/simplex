use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::BasicValueEnum;

use crate::constant::Constant;
use crate::errors::CompilationError;
use crate::scope::Scope;
use crate::types::{IntegerType, TypeSpec};
use crate::values::Value;

#[allow(unused)]
pub enum Expression {
    Constant(Constant),
    Identifier(Rc<str>),
    Conditional(Box<Expression>, Box<Expression>, Box<Expression>),
    BinaryOperation(BinaryOperationExpression),
    UnaryOperation(UnaryOperationExpression, Box<Expression>),
    Cast(Box<Expression>, TypeSpec),
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
    ) -> Result<Value<'ctx>, CompilationError> {
        let a = self.a.compile(scope, builder, ctx);
        let b = self.b.compile(scope, builder, ctx);
        assert_eq!(a.get_type(), b.get_type());

        // match a {
        //     Value::SignedInt(a_ir) => {}
        //     Value::UnsignedInt(a_ir) => {}
        // }

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
