use crate::ast;
use crate::expression::{BinaryOperation, Expression, ExpressionContext};
use crate::scope::LocalScope;
use crate::types::{IntegerType, TypeHint};

use inkwell::values::IntValue;

#[derive(Debug)]
pub enum IntegerExpression {
    Edge(Box<Expression>),
    LoadIntegerConstant(i32),
    BinaryOperation(IntegerBinaryOperation),
}

impl IntegerExpression {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        scope: &dyn LocalScope,
        type_hint: &TypeHint,
    ) -> Box<Self> {
        Box::new(match exp_ast {
            ast::Expression::BinaryOperation(exp_ast) => IntegerExpression::BinaryOperation(
                IntegerBinaryOperation::from_ast(exp_ast, scope, type_hint),
            ),
            ast::Expression::Constant(constant) => match constant {
                ast::Constant::Void => todo!(),
                ast::Constant::True => todo!(),
                ast::Constant::False => todo!(),
                ast::Constant::Integer(value) => IntegerExpression::LoadIntegerConstant(*value),
                ast::Constant::Float(_) => todo!(),
                ast::Constant::String(_) => todo!(),
            },
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::UnaryOperation(_) => todo!(),
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
            exp_ast => IntegerExpression::Edge(Expression::from_ast(exp_ast, scope, type_hint)),
        })
    }

    // pub fn infer_type(&self, type_hint: &TypeHint) -> Type {
    //     match self {
    //         IntegerExpression::LoadArgument(arg) => arg.arg_type(),
    //         IntegerExpression::LoadValue(val) => val.value_type(),
    //         IntegerExpression::LoadIntegerConstant(_) => {
    //             Type::Primitive(PrimitiveType::Integer(IntegerType {
    //                 is_signed: true,
    //                 width: IntegerTypeSize::I64,
    //             }))
    //         }
    //         IntegerExpression::BinaryOperation(op_exp) => op_exp.infer_type(type_hint),
    //     }
    // }
}

impl<'ctx> IntegerExpression {
    pub fn compile(
        &self,
        exp_type: &IntegerType,
        ctx: &ExpressionContext<'ctx, '_, '_, '_>,
    ) -> IntValue<'ctx> {
        match self {
            IntegerExpression::LoadIntegerConstant(val) => self.load_integer_constant(*val, ctx),
            IntegerExpression::BinaryOperation(op_exp) => op_exp.compile(exp_type, ctx),
            IntegerExpression::Edge(exp) => exp.compile(ctx).into_int_value(),
        }
    }

    fn load_integer_constant(
        &self,
        value: i32,
        ctx: &ExpressionContext<'ctx, '_, '_, '_>,
    ) -> IntValue<'ctx> {
        ctx.backend_context.i64_type().const_int(value as u64, true)
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
        type_hint: &TypeHint,
    ) -> Self {
        let lhs = IntegerExpression::from_ast(&exp_ast.lhs, scope, type_hint);
        let rhs = IntegerExpression::from_ast(&exp_ast.rhs, scope, type_hint);
        let op = exp_ast.operation.clone();
        Self { op, lhs, rhs }
    }

    // fn infer_type(&self, type_hint: &TypeHint) -> Type {
    //     todo!()
    //     match type_hint {
    //         TypeHint::Integer(type_spec) => type_spec.clone(),
    //         TypeHint::Inferred => {
    //             let lhs_type = self.lhs.infer_type(type_hint);
    //             let rhs_type = self.rhs.infer_type(type_hint);
    //             assert_eq!(lhs_type, rhs_type);
    //             lhs_type
    //         }
    //     }
    // }
}

impl<'ctx> IntegerBinaryOperation {
    fn compile(
        &self,
        exp_type: &IntegerType,
        ctx: &ExpressionContext<'ctx, '_, '_, '_>,
    ) -> IntValue<'ctx> {
        let lhs_ir = self.lhs.compile(exp_type, ctx);
        let rhs_ir = self.rhs.compile(exp_type, ctx);
        let builder = &ctx.builder;

        match self.op.clone() {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Sub => builder.build_int_sub(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Mul => builder.build_int_mul(lhs_ir, rhs_ir, "").unwrap(),
            BinaryOperation::Div => {
                if exp_type.is_signed {
                    builder.build_int_signed_div(lhs_ir, rhs_ir, "").unwrap()
                } else {
                    builder.build_int_unsigned_div(lhs_ir, rhs_ir, "").unwrap()
                }
            }
            BinaryOperation::Mod => {
                if exp_type.is_signed {
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
}
