use std::ops::Deref;

use crate::constant::Constant;
use crate::errors::CompilationResult;
use crate::expression::{BinaryOperationExpression, Expression, UnaryOperationExpression};
use crate::integer_value::IntegerValue;
use crate::statement_translator::StatementTranslator;
use crate::types::Type;
use crate::value::Value;

#[repr(transparent)]
pub struct ExpressionTranslator<'ctx, 'm, 'f, 's> {
    parent: &'s StatementTranslator<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 's> Deref for ExpressionTranslator<'ctx, 'm, 'f, 's> {
    type Target = StatementTranslator<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f, 's> ExpressionTranslator<'ctx, 'm, 'f, 's> {
    pub fn new(
        parent: &'s StatementTranslator<'ctx, 'm, 'f>,
    ) -> ExpressionTranslator<'ctx, 'm, 'f, 's> {
        ExpressionTranslator { parent }
    }

    pub fn translate(
        &self,
        expression: &Expression,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        match expression {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::BinaryOperation(expression) => {
                self.translate_binary_operation(expression, expression_type)
            }
            Expression::UnaryOperation(expression) => {
                self.translate_unary_operation(expression, expression_type)
            }
        }
    }

    fn translate_constant(&self, constant: &Constant) -> CompilationResult<Value<'ctx>> {
        match constant {
            Constant::Integer(value) => Ok(Value::Integer(IntegerValue::from_constant(
                *value,
                self.context(),
            ))),
        }
    }

    fn translate_binary_operation(
        &self,
        expression: &BinaryOperationExpression,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs = self.translate(&expression.lhs, expression_type)?;
        let rhs = self.translate(&expression.rhs, expression_type)?;
        lhs.binary_operation(
            expression.operation,
            &rhs,
            self.builder(),
            self.context(),
            expression_type,
        )
    }

    fn translate_unary_operation(
        &self,
        expression: &UnaryOperationExpression,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let arg = self.translate(&expression.arg, expression_type)?;
        arg.unary_operation(
            expression.operation,
            self.builder(),
            self.context(),
            expression_type,
        )
    }
}
