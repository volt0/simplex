use crate::basic_block::BasicBlockCompiler;
use crate::function::FunctionArgument;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use std::ops::Deref;
use std::rc::Rc;

pub enum BinaryOperation {
    Add,
}

pub enum IntegerExpression {
    BinaryOperation(
        BinaryOperation,
        Box<IntegerExpression>,
        Box<IntegerExpression>,
    ),
    LoadArgument(Rc<FunctionArgument>),
}

pub enum Expression {
    Integer(IntegerExpression),
}

impl Expression {}

pub struct ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    pub basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 'b> Deref for ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    type Target = BasicBlockCompiler<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.basic_block_compiler
    }
}

impl<'ctx, 'm, 'f, 'b> ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    pub fn compile_integer_load_argument(&self, arg: Rc<FunctionArgument>) -> IntValue<'ctx> {
        self.load_argument(arg.as_ref()).into_int_value()
    }

    pub fn compile_integer_add(
        &self,
        lhs: &IntegerExpression,
        rhs: &IntegerExpression,
    ) -> IntValue<'ctx> {
        let lhs = self.compile_integer_expression(lhs);
        let rhs = self.compile_integer_expression(rhs);
        self.builder().build_int_add(lhs, rhs, "").unwrap()
    }

    pub fn compile_integer_expression(&self, exp: &IntegerExpression) -> IntValue<'ctx> {
        match exp {
            IntegerExpression::BinaryOperation(op, lhs, rhs) => {
                let lhs = lhs.as_ref();
                let rhs = rhs.as_ref();
                match op {
                    BinaryOperation::Add => self.compile_integer_add(lhs, rhs),
                }
            }
            IntegerExpression::LoadArgument(arg) => self.compile_integer_load_argument(arg.clone()),
        }
    }

    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        match exp {
            Expression::Integer(exp) => self.compile_integer_expression(exp).as_basic_value_enum(),
        }
    }
}
