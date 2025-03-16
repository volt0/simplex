use crate::backend::expression_compiler::{ExpressionCodegen, ExpressionCompiler};
use crate::expression::{BinaryOperation, BinaryOperationExpression, Expression, ExpressionNode};
use crate::function::FunctionArgument;
use crate::statement::ValueAssignment;
use crate::types::IntegerType;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use std::ops::Deref;

pub struct IntegerExpressionCodegen<'ctx, 'm, 'f, 'b, 'e> {
    exp_compiler: &'e ExpressionCompiler<'ctx, 'm, 'f, 'b>,
    exp_type: IntegerType,
}

impl<'ctx, 'm, 'f, 'b, 'e> Deref for IntegerExpressionCodegen<'ctx, 'm, 'f, 'b, 'e> {
    type Target = ExpressionCompiler<'ctx, 'm, 'f, 'b>;

    fn deref(&self) -> &Self::Target {
        &self.exp_compiler
    }
}

impl<'ctx, 'm, 'f, 'b, 'e> ExpressionCodegen<'ctx>
    for IntegerExpressionCodegen<'ctx, 'm, 'f, 'b, 'e>
{
    fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        self.compile_expression_node(exp).as_basic_value_enum()
    }
}

impl<'ctx, 'm, 'f, 'b, 'e> IntegerExpressionCodegen<'ctx, 'm, 'f, 'b, 'e> {
    pub fn new(
        exp_compiler: &'e ExpressionCompiler<'ctx, 'm, 'f, 'b>,
        exp_type: IntegerType,
    ) -> Self {
        IntegerExpressionCodegen {
            exp_compiler,
            exp_type,
        }
    }

    pub fn compile_expression_node(&self, exp: &ExpressionNode) -> IntValue<'ctx> {
        match exp {
            ExpressionNode::LoadArgument(arg) => self.load_argument(arg),
            ExpressionNode::LoadValue(val) => self.load_value_assignment(val),
            ExpressionNode::LoadIntegerConstant(val) => self.load_integer_constant(*val),
            ExpressionNode::BinaryOperation(op_exp) => self.compile_binary_operation(op_exp),
        }
    }

    fn compile_binary_operation(&self, binary_op: &BinaryOperationExpression) -> IntValue<'ctx> {
        let lhs_ir = self.compile_expression_node(binary_op.lhs.as_ref());
        let rhs_ir = self.compile_expression_node(binary_op.rhs.as_ref());
        let builder = self.builder();

        match binary_op.op.clone() {
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
        let ctx = self.context();
        ctx.i64_type().const_int(value as u64, true)
    }

    fn load_value_assignment(&self, val: &ValueAssignment) -> IntValue<'ctx> {
        let value = self.exp_compiler.load_value_assignment(val);
        value.into_int_value()
    }

    fn load_argument(&self, arg: &FunctionArgument) -> IntValue<'ctx> {
        let value = self.exp_compiler.load_argument(arg);
        value.into_int_value()
    }
}
