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
        type_hint: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        match expression {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::BinaryOperation(expression) => {
                self.translate_binary_operation(expression, type_hint)
            }
            Expression::UnaryOperation(expression) => {
                self.translate_unary_operation(expression, type_hint)
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
        type_hint: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs = self.translate(&expression.lhs, type_hint)?;
        let rhs = self.translate(&expression.rhs, type_hint)?;
        lhs.binary_operation(
            expression.operation,
            &rhs,
            self.builder(),
            self.context(),
            type_hint,
        )
    }

    fn translate_unary_operation(
        &self,
        expression: &UnaryOperationExpression,
        type_hint: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let arg = self.translate(&expression.arg, type_hint)?;
        arg.unary_operation(
            expression.operation,
            self.builder(),
            self.context(),
            type_hint,
        )
    }
}
