use crate::ast;
use crate::ast::{Constant, TypeSpec};
use crate::basic_block::BasicBlockCompiler;
use crate::function::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::ValueAssignment;
use crate::type_spec::TypeHint;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
    ShiftLeft,
    ShiftRight,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    LogicalAnd,
    LogicalOr,
}

pub enum IntegerExpression {
    LoadConstant(i32),
    BinaryOperation(
        BinaryOperation,
        Box<IntegerExpression>,
        Box<IntegerExpression>,
    ),
    Force(Box<ExpressionNode>),
}

pub enum Expression {
    Integer(IntegerExpression),
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
}

impl Expression {
    pub fn from_ast(expression_ast: &ast::Expression, scope: &dyn LocalScope) -> Box<Self> {
        Box::new(match expression_ast {
            ast::Expression::Identifier(name) => match scope.resolve(name).expect("not found") {
                LocalScopeItem::Argument(arg) => Expression::LoadArgument(arg),
                LocalScopeItem::Value(val) => Expression::LoadValue(val),
            },
            ast::Expression::BinaryOperation(binary_operation_ast) => {
                let lhs = Box::new(IntegerExpression::Force(Box::new(ExpressionNode {
                    inner: Expression::from_ast(&binary_operation_ast.lhs, scope),
                    inferred_type: Default::default(),
                })));
                let rhs = Box::new(IntegerExpression::Force(Box::new(ExpressionNode {
                    inner: Expression::from_ast(&binary_operation_ast.rhs, scope),
                    inferred_type: Default::default(),
                })));
                Expression::Integer(IntegerExpression::BinaryOperation(
                    binary_operation_ast.operation.clone(),
                    lhs,
                    rhs,
                ))
            }
            ast::Expression::Constant(constant) => match constant {
                Constant::Void => todo!(),
                Constant::True => todo!(),
                Constant::False => todo!(),
                Constant::Integer(value) => {
                    Expression::Integer(IntegerExpression::LoadConstant(*value))
                }
                Constant::Float(_) => todo!(),
                Constant::String(_) => todo!(),
            },
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::UnaryOperation(_) => todo!(),
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
        })
    }
}

pub struct ExpressionNode {
    inner: Box<Expression>,
    inferred_type: OnceCell<TypeSpec>,
}

impl ExpressionNode {
    pub fn from_ast(expression_ast: &ast::Expression, scope: &dyn LocalScope) -> Box<Self> {
        Box::new(ExpressionNode {
            inner: Expression::from_ast(expression_ast, scope),
            inferred_type: Default::default(),
        })
    }

    pub fn infer_type(&self, type_hint: &TypeHint) -> TypeSpec {
        todo!()
        // match self.inner.as_ref() {
        //     Expression::Integer(_) => {}
        //     Expression::LoadArgument(_) => {}
        //     Expression::LoadValue(_) => {}
        // }
    }
}

impl Into<Expression> for IntegerExpression {
    fn into(self) -> Expression {
        Expression::Integer(self)
    }
}

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

    pub fn compile_load_argument(&self, arg: Rc<FunctionArgument>) -> BasicValueEnum<'ctx> {
        self.load_argument(arg.as_ref())
    }

    pub fn compile_load_value(&self, val: Rc<ValueAssignment>) -> BasicValueEnum<'ctx> {
        let id = val.ir_id.get().unwrap();
        self.load_value(*id).unwrap()
    }

    pub fn compile_integer_binary_operation(
        &self,
        op: BinaryOperation,
        lhs: &IntegerExpression,
        rhs: &IntegerExpression,
    ) -> IntValue<'ctx> {
        let lhs = self.compile_integer_expression(lhs);
        let rhs = self.compile_integer_expression(rhs);
        let builder = self.builder();
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
    }

    pub fn compile_integer_force(&self, exp: &ExpressionNode) -> IntValue<'ctx> {
        self.compile_expression(exp).into_int_value()
    }

    pub fn compile_integer_expression(&self, exp: &IntegerExpression) -> IntValue<'ctx> {
        match exp {
            IntegerExpression::LoadConstant(value) => {
                let i64_type = self.context().i64_type();
                i64_type.const_int(*value as u64, false)
            }
            IntegerExpression::BinaryOperation(op, lhs, rhs) => {
                let lhs = lhs.as_ref();
                let rhs = rhs.as_ref();
                self.compile_integer_binary_operation(op.clone(), lhs, rhs)
            }
            IntegerExpression::Force(exp) => self.compile_integer_force(exp.as_ref()),
        }
    }

    pub fn compile_expression(&self, exp: &ExpressionNode) -> BasicValueEnum<'ctx> {
        match exp.inner.as_ref() {
            Expression::Integer(exp) => self.compile_integer_expression(exp).as_basic_value_enum(),
            Expression::LoadArgument(arg) => self.compile_load_argument(arg.clone()),
            Expression::LoadValue(val) => self.compile_load_value(val.clone()),
        }
    }
}
