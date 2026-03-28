use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::errors::CompilationError;
use crate::value::{IntegerValue, Value};

pub enum Expression {
    LoadConstant(Constant),
    LoadValue(String),
    BinaryOperation(BinaryOperationExpression),
    UnaryOperation(UnaryOperationExpression),
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
        operation: BinaryOperation,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    ) -> Box<Self> {
        Box::new(Expression::BinaryOperation(BinaryOperationExpression {
            operation,
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

    fn new_unary_operation(operation: UnaryOperation, arg: Box<Expression>) -> Box<Self> {
        Box::new(Expression::UnaryOperation(UnaryOperationExpression {
            operation,
            arg,
        }))
    }
}

pub enum Constant {
    Integer(i32),
}

impl Constant {
    pub fn new_integer(value: i32) -> Self {
        Constant::Integer(value)
    }
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
    pub operation: BinaryOperation,
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
    pub operation: UnaryOperation,
    pub arg: Box<Expression>,
}

pub struct ExpressionTranslator<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub values: HashMap<String, Value<'ctx>>,
}

impl<'ctx> ExpressionTranslator<'ctx> {
    pub fn translate(&self, expression: &Expression) -> Result<Value<'ctx>, CompilationError> {
        match expression {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::BinaryOperation(expression) => self.translate_binary_operation(expression),
            Expression::UnaryOperation(expression) => self.translate_unary_operation(expression),
        }
    }

    fn load_value(&self, name: &str) -> Result<Value<'ctx>, CompilationError> {
        self.values
            .get(name)
            .ok_or(CompilationError::UnresolvedName(name.to_string()))
            .cloned()
    }

    fn translate_constant(&self, constant: &Constant) -> Result<Value<'ctx>, CompilationError> {
        match constant {
            Constant::Integer(value) => Ok(Value::Integer(IntegerValue::from_constant(
                *value,
                self.context,
            ))),
        }
    }

    fn translate_binary_operation(
        &self,
        expression: &BinaryOperationExpression,
    ) -> Result<Value<'ctx>, CompilationError> {
        let lhs = self.translate(&expression.lhs)?;
        let rhs = self.translate(&expression.rhs)?;
        lhs.binary_operation(expression.operation, &rhs, &self.builder, self.context)
    }

    fn translate_unary_operation(
        &self,
        expression: &UnaryOperationExpression,
    ) -> Result<Value<'ctx>, CompilationError> {
        let arg = self.translate(&expression.arg)?;
        arg.unary_operation(expression.operation, &self.builder, self.context)
    }
}
