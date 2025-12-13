use crate::expression::{
    BinaryOperation, ExpressionTranslator, Instruction, TypedExpressionTranslator,
};
use crate::types::Type;
use inkwell::values::{BasicValue, BasicValueEnum};

#[derive(Clone)]
pub struct Integer {
    pub is_signed: bool,
}

impl Type for Integer {
    fn create_expression_translator(&self) -> Box<dyn TypedExpressionTranslator> {
        Box::new(IntegerExpressionTranslator {
            integer_type: self.clone(),
        })
    }
}

struct IntegerExpressionTranslator {
    integer_type: Integer,
}

impl TypedExpressionTranslator for IntegerExpressionTranslator {
    fn translate_binary_operation<'ctx, 'm, 'f, 'b>(
        &self,
        op: &BinaryOperation,
        lhs: &Instruction,
        rhs: &Instruction,
        exp_translator: &ExpressionTranslator<'ctx, 'm, 'f, 'b>,
    ) -> BasicValueEnum<'ctx> {
        let lhs_ir = exp_translator
            .translate_instruction(lhs, self)
            .into_int_value();

        let rhs_ir = exp_translator
            .translate_instruction(rhs, self)
            .into_int_value();

        let builder = &exp_translator.builder;
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
    //         IntegerExpressionNode::UnaryOperation(op, arg) => {
    //             let arg_ir = self.translate_integer_node(arg, exp_type);
    //             match op {
    //                 UnaryOperation::Plus => arg_ir,
    //                 UnaryOperation::Minus => builder.build_int_neg(arg_ir, "").unwrap(),
    //                 UnaryOperation::BitwiseNot => builder.build_not(arg_ir, "").unwrap(),
    //             }
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
