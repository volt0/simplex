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
        expr: &Expression,
        expr_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let value = match expr {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::BinaryOperation(expr) => self.translate_binary_operation(expr, expr_type),
            Expression::UnaryOperation(expr) => self.translate_unary_operation(expr, expr_type),
            Expression::Call(expr) => self.translate_call(expr),
        };

        match expr_type {
            Some(expr_type) => value?.validate_type(expr_type, self),
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
        expr: &BinaryOperationExpression,
        expr_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let mut lhs = self.translate_expression(&expr.lhs, expr_type)?;
        let mut rhs = self.translate_expression(&expr.rhs, expr_type)?;

        if let None = expr_type {
            let combined_type = Type::combined_type(&lhs.value_type(), &rhs.value_type())?;
            lhs = lhs.validate_type(&combined_type, self)?;
            rhs = rhs.validate_type(&combined_type, self)?;
        }

        lhs.binary_operation(expr.op, &rhs, self)
    }

    fn translate_unary_operation(
        &self,
        expr: &UnaryOperationExpression,
        expr_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let arg = self.translate_expression(&expr.arg, expr_type)?;
        arg.unary_operation(expr.op, self)
    }

    fn translate_call(&self, expr: &CallExpression) -> CompilationResult<Value<'ctx>> {
        let callee = match self.translate_expression(&expr.callee, None)? {
            Value::Function(callee) => callee,
            _ => return Err(CompilationError::InvalidOperation),
        };

        let mut args_ir = Vec::with_capacity(expr.args.len());
        for (arg, arg_signature) in expr.args.iter().zip(callee.signature.args.iter()) {
            args_ir.push(
                self.translate_expression(arg, Some(&arg_signature.value_type))?
                    .into_basic_value_ir()?
                    .into(),
            );
        }

        let builder = self.builder();
        let callee_ir = callee.clone().into_ir();
        let result_ir = builder.build_call(callee_ir, args_ir.as_slice(), "")?;
        Ok(Value::from_ir(
            result_ir.as_any_value_enum(),
            &callee.signature.return_type,
        )?)
    }
}
