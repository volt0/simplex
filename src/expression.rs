use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::errors::CompilationError;
use crate::value::{IntegerValue, Value};

pub enum Expression {
    LoadConstant(Constant),
    LoadValue(String),
    BinaryOperation(BinaryOperationExpression),
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
}

pub struct BinaryOperationExpression {
    pub operation: BinaryOperation,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
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
}
