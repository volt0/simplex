use crate::expression::{Expression, ExpressionCompiler};
use crate::function::FunctionCompiler;
use crate::statement::Statement;
use inkwell::basic_block::BasicBlock as BasicBlockIR;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::values::BasicValueEnum;
use std::ops::Deref;

pub struct BasicBlock {
    pub statements: Vec<Statement>,
}

impl BasicBlock {}

pub struct BasicBlockCompiler<'ctx, 'm, 'f> {
    pub context: &'ctx Context,
    pub function_compiler: &'f FunctionCompiler<'ctx, 'm>,
    pub builder: Builder<'ctx>,
    pub basic_block: BasicBlockIR<'ctx>,
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionCompiler {
            builder: &self.builder,
            basic_block_compiler: self,
        };
        exp.compile(&exp_compiler)
    }

    pub fn compile_statement_return(&self, exp: &Expression) {
        let result = self.compile_expression(exp);
        self.builder.build_return(Some(&result)).unwrap();

        // let sum = self.builder.build_int_add(x, y, "sum").unwrap();
        // let sum = self.builder.build_int_add(sum, z, "sum").unwrap();
    }
}

impl<'ctx, 'm, 'f> Deref for BasicBlockCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.function_compiler
    }
}
