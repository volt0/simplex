use std::ops::Deref;

use crate::constant::Constant;
use crate::errors::{CompilationError, CompilationResult};
use crate::statement::StatementTranslator;
use crate::types::Type;
use crate::values::Value;

pub enum Expression {
    LoadConstant(Constant),
    LoadValue(String),
    BinaryOperation(BinaryOperationExpression),
    UnaryOperation(UnaryOperationExpression),
    Call(CallExpression),
}

#[derive(Copy, Clone)]
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
}

pub struct BinaryOperationExpression {
    pub op: BinaryOperation,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Copy, Clone)]
pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
}

pub struct UnaryOperationExpression {
    pub op: UnaryOperation,
    pub arg: Box<Expression>,
}

pub struct CallExpression {
    pub callee: Box<Expression>,
    pub args: Vec<Box<Expression>>,
}

impl Expression {
    pub fn new_load_constant(value: Constant) -> Box<Self> {
        Box::new(Expression::LoadConstant(value))
    }

    pub fn new_load_value(name: String) -> Box<Self> {
        Box::new(Expression::LoadValue(name))
    }

    pub fn new_add(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Add, lhs, rhs)
    }

    pub fn new_sub(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Sub, lhs, rhs)
    }

    pub fn new_mul(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Mul, lhs, rhs)
    }

    pub fn new_div(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Div, lhs, rhs)
    }

    pub fn new_mod(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::Mod, lhs, rhs)
    }

    pub fn new_bit_and(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::BitAnd, lhs, rhs)
    }

    pub fn new_bit_xor(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::BitXor, lhs, rhs)
    }

    pub fn new_bit_or(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::BitOr, lhs, rhs)
    }

    pub fn new_shift_left(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::ShiftLeft, lhs, rhs)
    }

    pub fn new_shift_right(lhs: Box<Expression>, rhs: Box<Expression>) -> Box<Self> {
        Self::new_binary_operation(BinaryOperation::ShiftRight, lhs, rhs)
    }

    fn new_binary_operation(
        op: BinaryOperation,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    ) -> Box<Self> {
        Box::new(Expression::BinaryOperation(BinaryOperationExpression {
            op,
            lhs,
            rhs,
        }))
    }

    pub fn new_unary_plus(arg: Box<Expression>) -> Box<Self> {
        Self::new_unary_operation(UnaryOperation::Plus, arg)
    }

    pub fn new_unary_minus(arg: Box<Expression>) -> Box<Self> {
        Self::new_unary_operation(UnaryOperation::Minus, arg)
    }

    pub fn new_bit_not(arg: Box<Expression>) -> Box<Self> {
        Self::new_unary_operation(UnaryOperation::BitNot, arg)
    }

    fn new_unary_operation(op: UnaryOperation, arg: Box<Expression>) -> Box<Self> {
        Box::new(Expression::UnaryOperation(UnaryOperationExpression {
            op,
            arg,
        }))
    }

    pub fn new_call(callee: Box<Expression>, args: Vec<Box<Expression>>) -> Box<Self> {
        Box::new(Expression::Call(CallExpression { callee, args }))
    }
}

#[repr(transparent)]
pub struct ExpressionTranslator<'ctx, 'm, 'f, 's> {
    parent: &'s StatementTranslator<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 's> ExpressionTranslator<'ctx, 'm, 'f, 's> {
    pub fn new(
        parent: &'s StatementTranslator<'ctx, 'm, 'f>,
    ) -> ExpressionTranslator<'ctx, 'm, 'f, 's> {
        ExpressionTranslator { parent }
    }

    pub fn translate_expression(
        &self,
        expr: &Expression,
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let value = match expr {
            Expression::LoadConstant(constant) => self.translate_constant(constant)?,
            Expression::LoadValue(name) => self.load_value(name)?,
            Expression::Call(expr) => self.translate_call(expr)?,
            Expression::BinaryOperation(expr) => {
                self.translate_binary_operation(expr.op, &expr.lhs, &expr.rhs, expr_type)?
            }
            Expression::UnaryOperation(expr) => {
                self.translate_unary_operation(expr.op, &expr.arg, expr_type)?
            }
        };

        if let Some(expr_type) = expr_type {
            expr_type.validate_value(self.builder(), &value)
        } else {
            Ok(value)
        }
    }

    fn translate_constant(&self, constant: &Constant) -> CompilationResult<Value<'ctx>> {
        Value::from_constant(self.context(), constant)
    }

    fn translate_binary_operation(
        &self,
        op: BinaryOperation,
        lhs_expr: &Expression,
        rhs_expr: &Expression,
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs = self.translate_expression(&lhs_expr, expr_type)?;
        let rhs = self.translate_expression(&rhs_expr, expr_type)?;
        lhs.do_binary_operation(self.builder(), op, &rhs)
    }

    fn translate_unary_operation(
        &self,
        op: UnaryOperation,
        arg_expr: &Expression,
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let arg = self.translate_expression(arg_expr, expr_type)?;
        arg.do_unary_operation(self.builder(), op)
    }

    fn translate_call(&self, expr: &CallExpression) -> CompilationResult<Value<'ctx>> {
        let callee = self.translate_expression(&expr.callee, None)?;
        let callee_type = callee.get_type();

        let arg_types = match callee_type {
            Type::Function(function_type) => function_type.arg_types().to_vec(),
            _ => return Err(CompilationError::TypeMismatch),
        };

        let mut args = Vec::with_capacity(arg_types.len());
        for (arg_expr, arg_type) in expr.args.iter().zip(arg_types.iter()) {
            args.push(self.translate_expression(arg_expr, Some(arg_type))?);
        }

        callee.do_call(self.builder(), &args)
    }
}

impl<'ctx, 'm, 'f, 's> Deref for ExpressionTranslator<'ctx, 'm, 'f, 's> {
    type Target = StatementTranslator<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}
