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
                let lhs = self.translate_expression(&expr.lhs, expr_type)?;
                let rhs = self.translate_expression(&expr.rhs, expr_type)?;
                self.translate_binary_operation(expr.op, lhs, rhs)
            }
            Expression::UnaryOperation(expr) => {
                let value = self.translate_expression(&expr.arg, expr_type)?;
                self.translate_unary_operation(expr.op, value)
            }
            Expression::Call(expr) => self.translate_call(expr),
        };

        if let Some(expr_type) = expr_type {
            self.type_check(value?, expr_type)
        } else {
            value
        }
    }

    pub fn type_check(
        &self,
        value: Value<'ctx>,
        value_type: &Type<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = self.builder();
        Ok(match value_type {
            Type::Integer(value_type) => match value {
                Value::Integer(value) => value.extend(builder, value_type)?.into(),
                Value::Bool(value) => value.to_integer(builder, value_type)?.into(),
                _ => return Err(CompilationError::TypeMismatch),
            },
            Type::Float(value_type) => match value {
                Value::Float(value) => value.extend(builder, value_type)?.into(),
                Value::Integer(value) => value.to_float(builder, value_type)?.into(),
                _ => return Err(CompilationError::TypeMismatch),
            },
            Type::Bool => match value {
                Value::Bool(value) => Value::Bool(value.clone()),
                Value::Integer(value) => value.to_bool(builder)?.into(),
                _ => return Err(CompilationError::TypeMismatch),
            },
        })
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
        lhs: Value<'ctx>,
        rhs: Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = self.builder();
        match lhs {
            Value::Integer(value) => value.binary_operation(builder, op, rhs.into_integer()),
            Value::Float(value) => value.binary_operation(builder, op, rhs.into_float()),
            Value::Bool(value) => value.binary_operation(builder, op, rhs.into_bool()),
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    fn translate_unary_operation(
        &self,
        op: UnaryOperation,
        value: Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        let builder = self.builder();
        match value {
            Value::Integer(value) => value.unary_operation(builder, op),
            Value::Float(value) => value.unary_operation(builder, op),
            Value::Bool(value) => value.unary_operation(builder, op),
            _ => Err(CompilationError::InvalidOperation),
        }
    }

    fn translate_call(&self, expr: &CallExpression) -> CompilationResult<Value<'ctx>> {
        let callee = match self.translate_expression(&expr.callee, None)? {
            Value::Function(callee) => callee,
            _ => return Err(CompilationError::InvalidOperation),
        };

        let mut args_ir = Vec::with_capacity(expr.args.len());
        for (arg, arg_signature) in expr.args.iter().zip(callee.signature.args.iter()) {
            let arg_type = Type::new(self.context(), arg_signature.value_type.clone());
            let arg_ir: BasicValueEnum<'ctx> = self
                .translate_expression(arg, Some(&arg_type))?
                .try_into()?;

            args_ir.push(BasicMetadataValueEnum::<'ctx>::from(arg_ir));
        }

        let builder = self.builder();
        let callee_ir = callee.clone().into();
        let result_ir = builder.build_call(callee_ir, args_ir.as_slice(), "")?;
        let return_type = Type::new(self.context(), callee.signature.return_type.clone());
        Value::from_ir(result_ir.as_any_value_enum(), &return_type)
    }
}
