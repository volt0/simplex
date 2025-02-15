use super::expression_compiler::ExpressionCompiler;
use super::function_compiler::FunctionCompiler;
use crate::basic_block::BasicBlockVisitor;
use crate::expression::ExpressionEdge;
use crate::statement::{Statement, ValueAssignment};
use inkwell::values::BasicValueEnum;
use std::ops::Deref;

#[repr(transparent)]
pub struct BasicBlockCompiler<'ctx, 'm, 'f> {
    parent: &'f FunctionCompiler<'ctx, 'm>,
}

impl<'ctx, 'm, 'f> Deref for BasicBlockCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f> BasicBlockVisitor for BasicBlockCompiler<'ctx, 'm, 'f> {
    fn visit_statement(&self, stmt: &Statement) {
        match stmt {
            Statement::ValueAssignment(var) => {
                self.compile_statement_let(var);
            }
            Statement::Return(exp) => {
                self.compile_statement_return(exp);
            }
        }
    }
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionCompiler<'ctx, 'm>) -> Self {
        Self { parent }
    }

    fn compile_statement_let(&self, val: &ValueAssignment) {
        let value = self.compile_expression(val.exp.as_ref());
        let value_id = self.store_value(value);
        val.ir_id.set(value_id).unwrap();
    }

    fn compile_statement_return(&self, exp: &ExpressionEdge) {
        let result = self.compile_expression(exp);
        self.builder().build_return(Some(&result)).unwrap();
    }

    fn compile_expression(&self, exp: &ExpressionEdge) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionCompiler::new(self);
        exp_compiler.compile_expression(exp)
    }
}
