use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::constant::Constant;
use crate::errors::CompilationError;
use crate::expression::{BinaryOperationExpression, Expression, UnaryOperationExpression};
use crate::integer_value::IntegerValue;
use crate::value::Value;

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
