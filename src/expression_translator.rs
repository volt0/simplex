use std::ops::Deref;

use inkwell::values::{AnyValue, BasicMetadataValueEnum, BasicValueEnum};

use crate::constant::Constant;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, CallExpression, Expression, UnaryOperation};
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
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let value = match expr {
            Expression::LoadConstant(constant) => self.translate_constant(constant),
            Expression::LoadValue(name) => self.load_value(name),
            Expression::BinaryOperation(expr) => {
                self.translate_binary_operation(expr.op, &expr.lhs, &expr.rhs, expr_type)
            }
            Expression::UnaryOperation(expr) => {
                self.translate_unary_operation(expr.op, &expr.arg, expr_type)
            }
            Expression::Call(expr) => self.translate_call(expr),
        };

        if let Some(expr_type) = expr_type {
            value?.validate_type(self.builder(), expr_type.clone())
        } else {
            value
        }
    }

    fn translate_constant(&self, constant: &Constant) -> CompilationResult<Value<'ctx>> {
        let context = self.context();
        Ok(match constant {
            Constant::Integer(value) => {
                Value::Integer(IntegerValue::from_constant(context, *value))
            }
        })
    }

    fn translate_binary_operation(
        &self,
        op: BinaryOperation,
        lhs_expr: &Expression,
        rhs_expr: &Expression,
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let lhs = self.translate_expression(&lhs_expr, expr_type)?;
        let rhs = self.translate_expression(&rhs_expr, expr_type)?;
        lhs.binary_operation(self.builder(), op, rhs)
    }

    fn translate_unary_operation(
        &self,
        op: UnaryOperation,
        arg_expr: &Expression,
        expr_type: Option<&Type<'ctx>>,
    ) -> CompilationResult<Value<'ctx>> {
        let arg = self.translate_expression(arg_expr, expr_type)?;
        arg.unary_operation(self.builder(), op)
    }

    fn translate_call(&self, expr: &CallExpression) -> CompilationResult<Value<'ctx>> {
        let callee = match self.translate_expression(&expr.callee, None)? {
            Value::Function(callee) => callee,
            _ => return Err(CompilationError::InvalidOperation),
        };

        let mut args_ir = Vec::with_capacity(expr.args.len());
        for (arg, arg_signature) in expr.args.iter().zip(callee.signature.args.iter()) {
            let arg_type = Type::from_spec(self.context(), arg_signature.value_type.clone());
            let arg_ir: BasicValueEnum<'ctx> = self
                .translate_expression(arg, Some(&arg_type))?
                .try_into()?;

            args_ir.push(BasicMetadataValueEnum::<'ctx>::from(arg_ir));
        }

        let builder = self.builder();
        let callee_ir = callee.clone().into();
        let result_ir = builder.build_call(callee_ir, args_ir.as_slice(), "")?;
        let return_type = Type::from_spec(self.context(), callee.signature.return_type.clone());
        Value::from_ir(result_ir.as_any_value_enum(), &return_type)
    }
}
