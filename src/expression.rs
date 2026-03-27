use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::value::{IntegerValue, Value};

pub enum Expression {
    LoadConstant(Constant),
    LoadValue(String),
    BinaryOperation(BinaryOperationExpression),
}

pub enum Constant {
    Integer(i32),
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
    pub fn translate(&self, expression: &Expression) -> Value<'ctx> {
        match expression {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::BinaryOperation(expression) => self.translate_binary_operation(expression),
        }
    }

    fn load_value(&self, name: &str) -> Value<'ctx> {
        self.values.get(name).unwrap().clone()
    }

    fn translate_constant(&self, constant: &Constant) -> Value<'ctx> {
        match constant {
            Constant::Integer(value) => {
                Value::Integer(IntegerValue::from_constant(*value, self.context))
            }
        }
    }

    fn translate_binary_operation(&self, expression: &BinaryOperationExpression) -> Value<'ctx> {
        let lhs = self.translate(&expression.lhs);
        let rhs = self.translate(&expression.rhs);
        lhs.binary_operation(expression.operation, &rhs, &self.builder, self.context)
    }
}
