use super::basic_block_compiler::BasicBlockCompiler;
use crate::backend::integer_expression::compile_integer_expression;
use crate::expression::Expression;
use crate::statement::ValueAssignment;
use crate::types::{PrimitiveType, Type};
use inkwell::values::BasicValueEnum;
use std::ops::Deref;

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

    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_type = exp.exp_type.clone();
        match exp_type {
            Type::Primitive(exp_type) => match exp_type {
                PrimitiveType::Void => todo!(),
                PrimitiveType::Bool => todo!(),
                PrimitiveType::Integer(int_type) => compile_integer_expression(exp, int_type, self),
                PrimitiveType::Float(_) => todo!(),
            },
        }
    }

    pub fn load_value_assignment(&self, val: &ValueAssignment) -> BasicValueEnum<'ctx> {
        let ir_id = val.ir_id.get().unwrap().clone();
        self.load_value(ir_id).unwrap()
    }
}
