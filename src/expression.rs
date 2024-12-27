use crate::ast;
use crate::basic_block::BasicBlockCompiler;
use crate::function::FunctionArgument;
use crate::statement::Value;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub enum BinaryOperation {
    Add,
}

pub enum IntegerExpression {
    BinaryOperation(
        BinaryOperation,
        Box<IntegerExpression>,
        Box<IntegerExpression>,
    ),
    Force(Box<Expression>),
}

pub enum Expression {
    Integer(IntegerExpression),
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<Value>),
}

impl Expression {
    pub fn from_ast(expression_ast: &ast::Expression) -> Box<Self> {
        match expression_ast {
            ast::Expression::Identifier(name) => {
                todo!()
            }
            ast::Expression::BinaryOperation(binary_operation_ast) => {
                todo!()
            }
            _ => todo!(),
        }
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

    pub fn compile_load_value(&self, val: Rc<Value>) -> BasicValueEnum<'ctx> {
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
        }
    }

    pub fn compile_integer_force(&self, exp: &Expression) -> IntValue<'ctx> {
        self.compile_expression(exp).into_int_value()
    }

    pub fn compile_integer_expression(&self, exp: &IntegerExpression) -> IntValue<'ctx> {
        match exp {
            IntegerExpression::BinaryOperation(op, lhs, rhs) => {
                let lhs = lhs.as_ref();
                let rhs = rhs.as_ref();
                self.compile_integer_binary_operation(op.clone(), lhs, rhs)
            }
            IntegerExpression::Force(exp) => self.compile_integer_force(exp.as_ref()),
        }
    }

    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        match exp {
            Expression::Integer(exp) => self.compile_integer_expression(exp).as_basic_value_enum(),
            Expression::LoadArgument(arg) => self.compile_load_argument(arg.clone()),
            Expression::LoadValue(val) => self.compile_load_value(val.clone()),
        }
    }
}
