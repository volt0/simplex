use crate::ast;
use crate::expression::{Expression, ExpressionCompiler};
use crate::function::FunctionCompiler;
use crate::statement::{Statement, Value};
use inkwell::values::BasicValueEnum;
use std::ops::Deref;

#[repr(transparent)]
pub struct BasicBlock {
    pub statements: Vec<Statement>,
}

impl BasicBlock {
    pub fn from_ast(basic_block_ast: &Vec<ast::Statement>) -> Self {
        let mut basic_block = BasicBlock { statements: vec![] };
        for statement in basic_block_ast {
            basic_block.statements.push(Statement::from_ast(statement));
        }
        basic_block
    }
}

#[repr(transparent)]
pub struct BasicBlockCompiler<'ctx, 'm, 'f> {
    function_compiler: &'f FunctionCompiler<'ctx, 'm>,
}

impl<'ctx, 'm, 'f> Deref for BasicBlockCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.function_compiler
    }
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionCompiler::new(self);
        exp_compiler.compile_expression(exp)
    }

    pub fn compile_statement_let(&self, val: &Value) {
        let value = self.compile_expression(val.assigned_exp.as_ref());
        let value_id = self.store_value(value);
        val.ir_id.set(value_id).unwrap();
    }

    pub fn compile_statement_return(&self, exp: &Expression) {
        let result = self.compile_expression(exp);
        self.builder().build_return(Some(&result)).unwrap();
    }
}

impl BasicBlock {
    pub fn compile(&self, function_compiler: &FunctionCompiler) {
        let basic_block_compiler = BasicBlockCompiler { function_compiler };
        for statement in self.statements.iter() {
            match statement {
                Statement::Let(var) => {
                    basic_block_compiler.compile_statement_let(var.as_ref());
                }
                Statement::Return(exp) => {
                    basic_block_compiler.compile_statement_return(exp);
                }
            }
        }
    }
}
