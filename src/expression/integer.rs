use crate::ast;
use crate::expression::{BinaryOperation, ExpressionCompiler};
use crate::function::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::ValueAssignment;
use crate::types::{IntegerType, IntegerTypeSize, PrimitiveType, Type, TypeHint};

use inkwell::values::IntValue;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub enum IntegerExpression {
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
    LoadIntegerConstant(i32),
    BinaryOperation(IntegerBinaryOperation),
}

impl IntegerExpression {
    pub fn from_ast(
        expression_ast: &ast::Expression,
        scope: &dyn LocalScope,
        type_hint: &TypeHint,
    ) -> Box<Self> {
        Box::new(match expression_ast {
            ast::Expression::Identifier(name) => {
                let resolved = scope.resolve(name).expect("not found");
                match resolved {
                    LocalScopeItem::Argument(arg) => IntegerExpression::LoadArgument(arg),
                    LocalScopeItem::Value(val) => IntegerExpression::LoadValue(val),
                }
            }
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
        })
    }

    pub fn infer_type(&self, type_hint: &TypeHint) -> Type {
        match self {
            IntegerExpression::LoadArgument(arg) => arg.arg_type(),
            IntegerExpression::LoadValue(val) => val.value_type(),
            IntegerExpression::LoadIntegerConstant(_) => {
                Type::Primitive(PrimitiveType::Integer(IntegerType {
                    is_signed: true,
                    width: IntegerTypeSize::I64,
                }))
            }
            IntegerExpression::BinaryOperation(op_exp) => op_exp.infer_type(type_hint),
        }
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

    fn infer_type(&self, type_hint: &TypeHint) -> Type {
        todo!()
        // match type_hint {
        //     TypeHint::Integer(type_spec) => type_spec.clone(),
        //     TypeHint::Inferred => {
        //         let lhs_type = self.lhs.infer_type(type_hint);
        //         let rhs_type = self.rhs.infer_type(type_hint);
        //         assert_eq!(lhs_type, rhs_type);
        //         lhs_type
        //     }
        // }
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

    pub fn compile_expression_node(&self, exp: &IntegerExpression) -> IntValue<'ctx> {
        match exp {
            IntegerExpression::LoadArgument(arg) => self.load_argument(arg),
            IntegerExpression::LoadValue(val) => self.load_value_assignment(val),
            IntegerExpression::LoadIntegerConstant(val) => self.load_integer_constant(*val),
            IntegerExpression::BinaryOperation(op_exp) => self.compile_binary_operation(op_exp),
        }
    }

    fn compile_binary_operation(&self, binary_op: &IntegerBinaryOperation) -> IntValue<'ctx> {
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
