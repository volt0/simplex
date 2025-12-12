use crate::ast;
use crate::expression::{BinaryOperation, ExpressionCompiler};
use crate::scope::LocalScope;
use crate::types::IntegerType;

use std::ops::Deref;

use inkwell::values::IntValue;

#[derive(Debug)]
pub enum IntegerExpression {
    LoadIntegerConstant(i32),
    BinaryOperation(IntegerBinaryOperation),
}

impl IntegerExpression {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        scope: &dyn LocalScope,
        type_hint: Option<&IntegerType>,
    ) -> Box<Self> {
        todo!()
    }
}

#[derive(Debug)]
pub struct IntegerBinaryOperation {
    pub op: BinaryOperation,
    pub lhs: Box<IntegerExpression>,
    pub rhs: Box<IntegerExpression>,
}

impl IntegerBinaryOperation {
    fn from_ast(
        exp_ast: &ast::BinaryOperationExpr,
        scope: &dyn LocalScope,
        type_hint: Option<&IntegerType>,
    ) -> Self {
        todo!()
    }
}

pub struct IntegerExpressionCompiler<'ctx, 'm, 'f, 'b, 'e> {
    exp_compiler: &'e ExpressionCompiler<'ctx, 'm, 'f, 'b>,
    exp_type: IntegerType,
}

impl<'ctx, 'm, 'f, 'b, 'e> Deref for IntegerExpressionCompiler<'ctx, 'm, 'f, 'b, 'e> {
    type Target = ExpressionCompiler<'ctx, 'm, 'f, 'b>;

    fn deref(&self) -> &Self::Target {
        &self.exp_compiler
    }
}

impl<'ctx, 'm, 'f, 'b, 'e> IntegerExpressionCompiler<'ctx, 'm, 'f, 'b, 'e> {
    pub fn new(
        exp_compiler: &'e ExpressionCompiler<'ctx, 'm, 'f, 'b>,
        exp_type: IntegerType,
    ) -> Self {
        IntegerExpressionCompiler {
            exp_compiler,
            exp_type,
        }
    }

    pub fn compile_integer_expression(&self, exp: &IntegerExpression) -> IntValue<'ctx> {
        match exp {
            IntegerExpression::LoadIntegerConstant(val) => self.load_integer_constant(*val),
            IntegerExpression::BinaryOperation(op_exp) => {
                self.compile_integer_binary_operation(op_exp)
            }
        }
    }

    fn compile_integer_binary_operation(&self, op_exp: &IntegerBinaryOperation) -> IntValue<'ctx> {
        let lhs_ir = self.compile_integer_expression(&op_exp.lhs);
        let rhs_ir = self.compile_integer_expression(&op_exp.rhs);
        let builder = &self.builder;

        match op_exp.op.clone() {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Sub => builder.build_int_sub(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Mul => builder.build_int_mul(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Div => {
                if self.exp_type.is_signed {
                    builder.build_int_signed_div(lhs_ir, rhs_ir, "").unwrap()
                } else {
                    builder.build_int_unsigned_div(lhs_ir, rhs_ir, "").unwrap()
                }
            }
            BinaryOperation::Mod => {
                if self.exp_type.is_signed {
                    builder.build_int_signed_rem(lhs_ir, rhs_ir, "").unwrap()
                } else {
                    builder.build_int_unsigned_rem(lhs_ir, rhs_ir, "").unwrap()
                }
            }
            BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::ShiftLeft => todo!(),
            BinaryOperation::ShiftRight => todo!(),
            BinaryOperation::Eq => todo!(),
            BinaryOperation::Ne => todo!(),
            BinaryOperation::Gt => todo!(),
            BinaryOperation::Ge => todo!(),
            BinaryOperation::Lt => todo!(),
            BinaryOperation::Le => todo!(),
            BinaryOperation::LogicalAnd => todo!(),
            BinaryOperation::LogicalOr => todo!(),
        }
    }

    fn load_integer_constant(&self, value: i32) -> IntValue<'ctx> {
        self.context.i64_type().const_int(value as u64, true)
    }
}
