use std::rc::Rc;

use crate::ast;
use crate::constant::Constant;
use crate::function_argument::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::ValueAssignment;
use crate::types::TypeHint;

#[derive(Clone, Debug)]
pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
}

#[derive(Clone, Debug)]
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

pub enum Instruction {
    LoadConstant(Constant),
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
    BinaryOperation(BinaryOperation, Box<Instruction>, Box<Instruction>),
    UnaryOperation(UnaryOperation, Box<Instruction>),
    // TypeAssertedSubtree(Box<Expression>),
    // Truncate(Box<IntegerExpression>),
}

impl Instruction {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        type_hint: &TypeHint,
        scope: &dyn LocalScope,
    ) -> Instruction {
        match exp_ast {
            ast::Expression::Constant(const_ast) => {
                Instruction::LoadConstant(Constant::from_ast(const_ast))
            }
            ast::Expression::Identifier(name) => match scope.resolve(name).unwrap() {
                LocalScopeItem::Argument(arg) => Instruction::LoadArgument(arg),
                LocalScopeItem::Value(val) => Instruction::LoadValue(val),
            },
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::BinaryOperation(exp_ast) => {
                let lhs = Box::new(Self::from_ast(exp_ast.lhs.as_ref(), type_hint, scope));
                let rhs = Box::new(Self::from_ast(exp_ast.rhs.as_ref(), type_hint, scope));
                Instruction::BinaryOperation(exp_ast.operation.clone(), lhs, rhs)
            }
            ast::Expression::UnaryOperation(exp_ast) => {
                let arg = Box::new(Self::from_ast(exp_ast.arg.as_ref(), type_hint, scope));
                Instruction::UnaryOperation(exp_ast.operation.clone(), arg)
            }
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
        }
    }
}
