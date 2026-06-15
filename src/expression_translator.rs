use std::ops::Deref;

use inkwell::values::{AnyValue, BasicValueEnum};

use crate::constant::Constant;
use crate::errors::CompilationResult;
use crate::expression::{BinaryOperation, CallExpression, Expression, UnaryOperation};
use crate::statement_translator::StatementTranslator;
use crate::types::Type;
use crate::values::Value;

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
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let value = match expr {
            Expression::LoadConstant(constant) => self.translate_constant(constant)?,
            Expression::LoadValue(name) => self.load_value(name)?,
            Expression::Call(expr) => self.translate_call(expr)?,
            Expression::BinaryOperation(expr) => {
                self.translate_binary_operation(expr.op, &expr.lhs, &expr.rhs, expr_type)?
            }
            Expression::UnaryOperation(expr) => {
                self.translate_unary_operation(expr.op, &expr.arg, expr_type)?
            }
        };

        if let Some(expr_type) = expr_type {
            expr_type.check_value(self.builder(), value)
        } else {
            Ok(value)
        }
    }

    fn translate_constant(&self, constant: &Constant) -> CompilationResult<Value<'ctx>> {
        Value::from_constant(self.context(), constant)
    }

    fn translate_binary_operation(
        &self,
        op: BinaryOperation,
        lhs_expr: &Expression,
        rhs_expr: &Expression,
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let mut lhs = self.translate_expression(&lhs_expr, expr_type)?;
        let mut rhs = self.translate_expression(&rhs_expr, expr_type)?;
        lhs.do_binary_operation(self.builder(), op, &mut rhs)?;
        Ok(lhs)
    }

    fn translate_unary_operation(
        &self,
        op: UnaryOperation,
        arg_expr: &Expression,
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let mut arg = self.translate_expression(arg_expr, expr_type)?;
        arg.do_unary_operation(self.builder(), op)?;
        Ok(arg)
    }

    fn translate_call(&self, expr: &CallExpression) -> CompilationResult<Value<'ctx>> {
        let callee = self.translate_expression(&expr.callee, None)?;
        let callee = callee.to_function()?;

        let callee_type = callee.get_type();
        let callee_ir = callee.ir().clone();

        let mut args_ir = Vec::with_capacity(expr.args.len());
        for (arg_expr, arg_type) in expr.args.iter().zip(callee_type.arg_types().iter()) {
            let arg_ir: BasicValueEnum = self
                .translate_expression(arg_expr, Some(arg_type))?
                .try_into()?;
            args_ir.push(arg_ir.into());
        }

        let builder = self.builder();
        let result_ir = builder.build_call(callee_ir, args_ir.as_slice(), "")?;
        let return_type = callee_type.return_type();
        Value::from_ir(result_ir.as_any_value_enum(), return_type)
    }
}
