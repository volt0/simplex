use super::basic_block_compiler::BasicBlockCompiler;
use crate::expression::{
    BinaryOperation, BinaryOperationExpression, ExpressionEdge, ExpressionNode,
};
use crate::function::FunctionArgument;
use crate::statement::ValueAssignment;
use crate::types::Type;
use inkwell::values::{BasicValue, BasicValueEnum};
use std::ops::Deref;

#[repr(transparent)]
pub struct ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 'b> Deref for ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    type Target = BasicBlockCompiler<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.basic_block_compiler
    }
}

impl<'ctx, 'm, 'f, 'b> ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    pub fn new(basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>) -> Self {
        ExpressionCompiler::<'ctx, 'm, 'f, 'b> {
            basic_block_compiler,
        }
    }

    pub fn compile_expression(&self, exp: &ExpressionEdge) -> BasicValueEnum<'ctx> {
        let exp_type = exp.exp_type();
        match exp.node() {
            ExpressionNode::LoadArgument(arg) => self.compile_load_argument(arg),
            ExpressionNode::LoadValue(val) => self.compile_load_value(val),
            ExpressionNode::LoadIntegerConstant(val) => self.compile_load_integer_constant(*val),
            ExpressionNode::BinaryOperation(op_exp) => {
                self.compile_binary_operation(op_exp, exp_type)
            }
        }
    }

    fn compile_load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.load_argument(arg)
    }

    fn compile_load_value(&self, val: &ValueAssignment) -> BasicValueEnum<'ctx> {
        let ir_id = val.ir_id.get().unwrap().clone();
        self.load_value(ir_id).unwrap()
    }

    fn compile_load_integer_constant(&self, value: i32) -> BasicValueEnum<'ctx> {
        let ctx = self.context();
        ctx.i64_type()
            .const_int(value as u64, true)
            .as_basic_value_enum()
    }

    fn compile_binary_operation(
        &self,
        binary_op: &BinaryOperationExpression,
        type_spec: Type,
    ) -> BasicValueEnum<'ctx> {
        let lhs_ir = self.compile_expression(binary_op.lhs.as_ref());
        let rhs_ir = self.compile_expression(binary_op.rhs.as_ref());

        // fixme: if let Type::I64 = type_spec {}
        _ = type_spec;
        self.compile_integer_binary_operation(binary_op.op.clone(), lhs_ir, rhs_ir)
    }

    pub fn compile_integer_binary_operation(
        &self,
        op: BinaryOperation,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> BasicValueEnum<'ctx> {
        let builder = self.builder();
        let lhs = lhs.into_int_value();
        let rhs = rhs.into_int_value();
        match op {
            BinaryOperation::Add => builder.build_int_add(lhs, rhs, "").unwrap(),
            BinaryOperation::Sub => todo!(),
            BinaryOperation::Mul => todo!(),
            BinaryOperation::Div => todo!(),
            BinaryOperation::Mod => todo!(),
            BinaryOperation::BitAnd => todo!(),
            BinaryOperation::BitXor => todo!(),
            BinaryOperation::BitOr => todo!(),
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
        .as_basic_value_enum()
    }
}
