use std::collections::HashMap;
use std::ops::Deref;

use crate::constant::Constant;
use crate::expression::{BinaryOperationExpression, Expression, UnaryOperationExpression};
use crate::integer_value::IntegerValue;
use crate::statement_translator::StatementTranslator;
use crate::value::Value;

pub struct ExpressionTranslator<'ctx> {
    pub parent: &'ctx StatementTranslator<'ctx>,
    pub values: HashMap<String, Value<'ctx>>,
}

impl<'ctx> Deref for ExpressionTranslator<'ctx> {
    type Target = StatementTranslator<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx> ExpressionTranslator<'ctx> {
    pub fn translate_expression(&self, expression: &Expression) -> Value<'ctx> {
        match expression {
            Expression::LoadConstant(value) => self.translate_constant(value),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::Conditional(_) => todo!(),
            Expression::UnaryOperation(expression) => self.translate_unary_operation(expression),
            Expression::BinaryOperation(expression) => self.translate_binary_operation(expression),
            Expression::LogicalOperation(_) => todo!(),
            Expression::Cast(_) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::ItemAccess(_) => todo!(),
            Expression::MemberAccess(_) => todo!(),
        }
    }

    fn translate_constant(&self, value: &Constant) -> Value<'ctx> {
        match value {
            Constant::Void => todo!(),
            Constant::True => todo!(),
            Constant::False => todo!(),
            Constant::Integer(value) => Value::IntegerValue(IntegerValue {
                ir: self.context.i32_type().const_int(*value as u64, false),
            }),
            Constant::Float(_) => todo!(),
            Constant::String(_) => todo!(),
        }
    }

    fn load_value(&self, name: &str) -> Value<'ctx> {
        self.values
            .get(name)
            .cloned()
            .unwrap_or_else(|| self.parent.load_value(name))
    }

    fn translate_unary_operation(&self, expression: &UnaryOperationExpression) -> Value<'ctx> {
        let arg = self.translate_expression(&expression.arg);
        arg.unary_operation(expression.operation.clone(), &self.builder)
    }

    fn translate_binary_operation(&self, expression: &BinaryOperationExpression) -> Value<'ctx> {
        let left = self.translate_expression(&expression.lhs);
        let right = self.translate_expression(&expression.rhs);
        left.binary_operation(expression.operation.clone(), right, &self.builder)
    }
}
