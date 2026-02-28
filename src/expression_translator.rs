use std::collections::HashMap;
use std::ops::Deref;

use crate::constant::Constant;
use crate::expression::Expression;
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::integer_value::IntegerValue;
use crate::statement_translator::StatementTranslator;
use crate::type_spec::TypeHint;
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
    pub fn translate_expression(
        &self,
        expression: &Expression,
        type_hint: &TypeHint,
    ) -> Value<'ctx> {
        let result = match expression {
            Expression::LoadConstant(value) => self.translate_constant(value),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::Conditional(_) => todo!(),

            Expression::UnaryOperation(expression) => {
                let arg = self.translate_expression(&expression.arg, type_hint);
                arg.unary_operation(expression.operation.clone(), &self.builder)
            }

            Expression::BinaryOperation(expression) => {
                let lhs = self.translate_expression(&expression.lhs, type_hint);
                let rhs = self.translate_expression(&expression.rhs, &Some(lhs.get_type()));
                lhs.binary_operation(expression.operation.clone(), rhs, &self.builder)
            }

            Expression::LogicalOperation(_) => todo!(),
            Expression::Cast(_) => todo!(),
            Expression::Call(_) => todo!(),
            Expression::ItemAccess(_) => todo!(),
            Expression::MemberAccess(_) => todo!(),
        };

        if let Some(type_hint) = type_hint {
            result.type_check(type_hint)
        } else {
            result
        }
    }

    fn translate_constant(&self, value: &Constant) -> Value<'ctx> {
        match value {
            Constant::Void => todo!(),
            Constant::True => todo!(),
            Constant::False => todo!(),
            Constant::Integer(value) => Value::IntegerValue(IntegerValue {
                ir: self.context.i32_type().const_int(*value as u64, false),
                value_type: IntegerType {
                    is_signed: false,
                    width: IntegerTypeSize::I32,
                },
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
}
