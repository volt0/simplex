use crate::basic_block::BasicBlockCompiler;
use crate::function::FunctionArgument;
use crate::value::Value;
use inkwell::builder::Builder;
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

impl<'ctx, 'm, 'f, 'b> IntegerExpression {
    pub fn compile(&self, compiler: &ExpressionCompiler<'ctx, 'm, 'f, 'b>) -> IntValue<'ctx> {
        match self {
            IntegerExpression::BinaryOperation(_, _, _) => {
                todo!()
            }
            IntegerExpression::LoadArgument(arg) => {
                todo!()
                // compiler.compile_integer_load_argument(arg.clone())
                // compiler.compile_integer_expression(exp)
            }
        }
    }
}

pub enum Expression {
    Integer(IntegerExpression),
}

impl Expression {
    pub fn compile<'ctx>(&self, compiler: &ExpressionCompiler) -> BasicValueEnum<'ctx> {
        todo!()
        // match self {
        //     Expression::Integer(exp) => compiler
        //         .compile_integer_expression(exp)
        //         .as_basic_value_enum(),
        // }
    }
}

pub struct ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    pub builder: &'b Builder<'ctx>,
    pub basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 'b> ExpressionCompiler<'ctx, 'm, 'f, 'b>
where
    'm: 'ctx,
    'f: 'm,
    'b: 'f,
{
    pub fn compile_integer_load_argument(&self, arg: Rc<FunctionArgument>) -> IntValue<'ctx> {
        if let Value::Integer(arg_ir) = self.load_value(arg.ir_id.get().unwrap().clone()).unwrap() {
            return arg_ir;
        }
        
        panic!()
    }

    pub fn compile_integer_expression(&self, exp: &IntegerExpression) -> IntValue<'ctx> {
        match exp {
            IntegerExpression::BinaryOperation(_, _, _) => todo!(),
            IntegerExpression::LoadArgument(arg) => {
                self.compile_integer_load_argument(arg.clone());
                self.load_value(arg.as_ref().ir_id.get().unwrap().clone())
                    .unwrap();
                todo!()
            }
        }
    }

    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        match exp {
            Expression::Integer(exp) => self.compile_integer_expression(exp).as_basic_value_enum(),
        }
    }
}

impl<'ctx, 'm, 'f, 'b> Deref for ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    type Target = BasicBlockCompiler<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.basic_block_compiler
    }
}
