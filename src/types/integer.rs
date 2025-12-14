use std::ops::Deref;

use inkwell::values::{BasicValue, BasicValueEnum};

use crate::ast;
use crate::expression::{BinaryOperation, ExpressionTranslator, Instruction, UnaryOperation};

#[derive(Clone, Debug, PartialEq)]
pub struct IntegerType {
    pub is_signed: bool,
    pub width: IntegerTypeSize,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
}

impl IntegerType {
    pub fn from_ast(int_type_ast: &ast::IntegerType) -> Self {
        match int_type_ast {
            ast::IntegerType::I8 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I8,
            },
            ast::IntegerType::I16 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I16,
            },
            ast::IntegerType::I32 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I32,
            },
            ast::IntegerType::I64 => IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I64,
            },
            ast::IntegerType::U8 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I8,
            },
            ast::IntegerType::U16 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I16,
            },
            ast::IntegerType::U32 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I32,
            },
            ast::IntegerType::U64 => IntegerType {
                is_signed: false,
                width: IntegerTypeSize::I64,
            },
        }
    }
}

pub struct IntegerExpressionTranslator<'ctx, 'm, 'f, 'b, 'e> {
    parent: &'e ExpressionTranslator<'ctx, 'm, 'f, 'b>,
    integer_type: IntegerType,
}

impl<'ctx, 'm, 'f, 'b, 'e> Deref for IntegerExpressionTranslator<'ctx, 'm, 'f, 'b, 'e> {
    type Target = ExpressionTranslator<'ctx, 'm, 'f, 'b>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f, 'b, 'e> IntegerExpressionTranslator<'ctx, 'm, 'f, 'b, 'e> {
    pub fn new(
        parent: &'b ExpressionTranslator<'ctx, 'm, 'f, 'b>,
        integer_type: IntegerType,
    ) -> Self {
        IntegerExpressionTranslator::<'ctx, 'm, 'f, 'b, 'e> {
            parent,
            integer_type,
        }
    }

    pub fn translate_instruction(&self, instruction: &Instruction) -> BasicValueEnum<'ctx> {
        match instruction {
            Instruction::LoadConstant(const_value) => self.translate_constant(const_value),
            Instruction::UnaryOperation(op, arg) => self.translate_unary_operation(op, arg),
            Instruction::BinaryOperation(op, lhs, rhs) => {
                self.translate_binary_operation(op, lhs, rhs)
            }
        }
    }

    fn translate_unary_operation(
        &self,
        op: &UnaryOperation,
        arg: &Instruction,
    ) -> BasicValueEnum<'ctx> {
        let arg_ir = self.translate_instruction(arg).into_int_value();

        let builder = &self.parent.builder;
        let result = match op {
            UnaryOperation::Plus => arg_ir,
            UnaryOperation::Minus => builder.build_int_neg(arg_ir, "").unwrap(),
            UnaryOperation::BitNot => builder.build_not(arg_ir, "").unwrap(),
            UnaryOperation::LogicalNot => todo!(),
        };

        result.as_basic_value_enum()
    }

    fn translate_binary_operation(
        &self,
        op: &BinaryOperation,
        lhs: &Instruction,
        rhs: &Instruction,
    ) -> BasicValueEnum<'ctx> {
        let lhs_ir = self.translate_instruction(lhs).into_int_value();
        let rhs_ir = self.translate_instruction(rhs).into_int_value();

        let builder = &self.parent.builder;
        let result = match op {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Sub => builder.build_int_sub(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Mul => builder.build_int_mul(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Div => {
                if self.integer_type.is_signed {
                    builder.build_int_signed_div(lhs_ir, rhs_ir, "").unwrap()
                } else {
                    builder.build_int_unsigned_div(lhs_ir, rhs_ir, "").unwrap()
                }
            }
            BinaryOperation::Mod => {
                if self.integer_type.is_signed {
                    builder.build_int_signed_rem(lhs_ir, rhs_ir, "").unwrap()
                } else {
                    builder.build_int_unsigned_rem(lhs_ir, rhs_ir, "").unwrap()
                }
            }
            BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::ShiftLeft => builder.build_left_shift(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::ShiftRight => builder
                .build_right_shift(lhs_ir, rhs_ir, self.integer_type.is_signed, "")
                .unwrap(),
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

    // fn translate_integer_node(
    //     &self,
    //     node: &IntegerExpressionNode,
    //     exp_type: &IntegerType,
    // ) -> IntValue<'ctx> {
    //     match node {
    //         IntegerExpressionNode::TypeAssertedSubtree(exp) => {
    //             self.translate_expression(exp.as_ref()).into_int_value()
    //         }
    //         IntegerExpressionNode::Truncate(exp) => {
    //             let value = self.translate_integer_expression(exp.as_ref());
    //             builder
    //                 .build_int_truncate(value, exp_type.compile(ctx), "")
    //                 .unwrap()
    //         }
    //         IntegerExpressionNode::Cast(exp) => {
    //             let value = self.translate_integer_expression(exp.as_ref());
    //             let type_ir = exp_type.compile(ctx);
    //
    //             if exp_type.size > exp.exp_type.size {
    //                 if exp_type.is_signed {
    //                     builder.build_int_s_extend(value, type_ir, "").unwrap()
    //                 } else {
    //                     builder.build_int_z_extend(value, type_ir, "").unwrap()
    //                 }
    //             } else {
    //                 builder.build_int_truncate(value, type_ir, "").unwrap()
    //             }
    //         }
    //     }
    // }
}
