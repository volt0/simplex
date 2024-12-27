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
    context: &'ctx Context,
    function_compiler: &'f FunctionCompiler<'ctx, 'm>,
    builder: &'f Builder<'ctx>,
    basic_block: BasicBlockIR<'ctx>,
}

impl<'ctx, 'm, 'f> Deref for BasicBlockCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.function_compiler
    }
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionCompiler {
            basic_block_compiler: self,
        };

        exp_compiler.compile_expression(exp)
    }

    pub fn compile_statement_return(&self, exp: &Expression) {
        let result = self.compile_expression(exp);
        self.builder.build_return(Some(&result)).unwrap();
    }
}

pub fn compile_basic_block(basic_block: &BasicBlock, function_compiler: &FunctionCompiler) {
    let basic_block_ir = function_compiler.add_basic_block();

    let basic_block_compiler = BasicBlockCompiler {
        context: function_compiler.context(),
        builder: function_compiler.builder(),
        function_compiler,
        basic_block: basic_block_ir,
    };

    for statement in basic_block.statements.iter() {
        match statement {
            Statement::Return(exp) => {
                basic_block_compiler.compile_statement_return(exp);
            }
        }
    }
}
