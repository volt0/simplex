use super::basic_block_compiler::BasicBlockCompiler;
use crate::expression::{
    BinaryOperation, BinaryOperationExpression, ExpressionEdge, ExpressionNode,
};
use crate::statement::ValueAssignment;
use crate::types::{IntegerType, PrimitiveType, Type};
use inkwell::builder::Builder;
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

    pub fn compile_expression_edge(&self, exp: &ExpressionEdge) -> BasicValueEnum<'ctx> {
        let exp_type = exp.exp_type();
        self.compile_expression_node(exp, exp_type)
    }

    fn compile_expression_node(
        &self,
        exp: &ExpressionNode,
        exp_type: Type,
    ) -> BasicValueEnum<'ctx> {
        match exp {
            ExpressionNode::LoadArgument(arg) => self.load_argument(arg),
            ExpressionNode::LoadValue(val) => self.load_value_assignment(val),
            ExpressionNode::LoadIntegerConstant(val) => self.load_integer_constant(*val),
            ExpressionNode::BinaryOperation(op_exp) => {
                self.compile_binary_operation(op_exp, exp_type)
            }
        }
    }

    fn compile_binary_operation(
        &self,
        binary_op: &BinaryOperationExpression,
        exp_type: Type,
    ) -> BasicValueEnum<'ctx> {
        let lhs_ir = self.compile_expression_edge(binary_op.lhs.as_ref());
        let rhs_ir = self.compile_expression_edge(binary_op.rhs.as_ref());
        let builder = self.builder();
        compile_binary_operation(exp_type, binary_op.op.clone(), lhs_ir, rhs_ir, builder)
    }

    fn load_integer_constant(&self, value: i32) -> BasicValueEnum<'ctx> {
        let ctx = self.context();
        ctx.i64_type()
            .const_int(value as u64, true)
            .as_basic_value_enum()
    }

    fn load_value_assignment(&self, val: &ValueAssignment) -> BasicValueEnum<'ctx> {
        let ir_id = val.ir_id.get().unwrap().clone();
        self.load_value(ir_id).unwrap()
    }
}

fn compile_binary_operation<'ctx>(
    exp_type: Type,
    op: BinaryOperation,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
    builder: &Builder<'ctx>,
) -> BasicValueEnum<'ctx> {
    match exp_type {
        Type::Primitive(primitive_type) => match primitive_type {
            PrimitiveType::Void => todo!(),
            PrimitiveType::Bool => todo!(),
            PrimitiveType::Integer(int_type) => {
                compile_int_binary_operation(int_type, op, lhs, rhs, builder)
            }
            PrimitiveType::Float(_) => todo!(),
        },
    }
}

fn compile_int_binary_operation<'ctx>(
    exp_type: IntegerType,
    op: BinaryOperation,
    lhs: BasicValueEnum<'ctx>,
    rhs: BasicValueEnum<'ctx>,
    builder: &Builder<'ctx>,
) -> BasicValueEnum<'ctx> {
    let lhs = lhs.into_int_value();
    let rhs = rhs.into_int_value();
    let result = match op {
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
    };
    result.as_basic_value_enum()
}
