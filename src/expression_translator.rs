use std::ops::Deref;

use inkwell::values::AnyValue;

use crate::constant::Constant;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{
    BinaryOperationExpression, CallExpression, Expression, UnaryOperationExpression,
};
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

    pub fn translate_expression(
        &self,
        expression: &Expression,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let value = match expression {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::BinaryOperation(expression) => {
                self.translate_binary_operation(expression, expression_type)
            }
            Expression::UnaryOperation(expression) => {
                self.translate_unary_operation(expression, expression_type)
            }
            Expression::Call(expression) => self.translate_call(expression),
        };

        match expression_type {
            Some(expression_type) => value?.validate_type(expression_type, self),
            None => value,
        }
    }

    fn translate_constant(&self, constant: &Constant) -> CompilationResult<Value<'ctx>> {
        match constant {
            Constant::Integer(value) => {
                Ok(Value::Integer(IntegerValue::from_constant(*value, self)))
            }
        }
    }

    fn translate_binary_operation(
        &self,
        expression: &BinaryOperationExpression,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let mut lhs = self.translate_expression(&expression.lhs, expression_type)?;
        let mut rhs = self.translate_expression(&expression.rhs, expression_type)?;

        if let None = expression_type {
            let common_type = Type::combined_type(&lhs.value_type(), &rhs.value_type())?;
            lhs = lhs.validate_type(&common_type, self)?;
            rhs = rhs.validate_type(&common_type, self)?;
        }

        lhs.binary_operation(expression.operation, &rhs, self)
    }

    fn translate_unary_operation(
        &self,
        expression: &UnaryOperationExpression,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let arg = self.translate_expression(&expression.arg, expression_type)?;
        arg.unary_operation(expression.operation, self)
    }

    fn translate_call(&self, expression: &CallExpression) -> CompilationResult<Value<'ctx>> {
        let callee = match self.translate_expression(&expression.callee, None)? {
            Value::Function(callee) => callee,
            _ => return Err(CompilationError::InvalidOperation),
        };

        let mut arguments_ir = Vec::with_capacity(expression.arguments.len());
        for (argument, arg_signature) in expression
            .arguments
            .iter()
            .zip(callee.signature.args.iter())
        {
            arguments_ir.push(
                self.translate_expression(argument, Some(&arg_signature.value_type))?
                    .into_basic_value_ir()?
                    .into(),
            );
        }

        let builder = self.builder();
        let callee_ir = callee.clone().into_ir();
        let result_ir = builder.build_call(callee_ir, arguments_ir.as_slice(), "")?;
        Ok(Value::from_ir(
            result_ir.as_any_value_enum(),
            &callee.signature.return_type,
        )?)
    }
}
