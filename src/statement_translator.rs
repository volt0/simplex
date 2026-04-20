use std::collections::HashMap;
use std::ops::Deref;

use inkwell::values::BasicValueEnum;

use crate::block::Block;
use crate::errors::CompilationResult;
use crate::expression::Expression;
use crate::expression_translator::ExpressionTranslator;
use crate::function_builder::FunctionBuilder;
use crate::statement::StatementVisitor;
use crate::value::Value;

pub struct StatementTranslator<'ctx, 'm, 'f> {
    parent: &'f FunctionBuilder<'ctx, 'm>,
    values: HashMap<String, Value<'ctx>>,
}

impl<'ctx, 'm, 'f> Deref for StatementTranslator<'ctx, 'm, 'f> {
    type Target = FunctionBuilder<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f> StatementVisitor for StatementTranslator<'ctx, 'm, 'f> {
    fn enter_block(&self, block: &Block) -> CompilationResult<()> {
        block.visit(self)
    }

    fn add_return_statement(&self, expr: &Expression) -> CompilationResult<()> {
        let expr_translator = ExpressionTranslator::new(self);
        let expr_type = self.function_return_type().clone();

        let value = expr_translator.translate_expression(expr, Some(&expr_type))?;
        let value_ir: BasicValueEnum<'ctx> = value.try_into()?;

        self.builder().build_return(Some(&value_ir))?;
        Ok(())
    }
}

impl<'ctx, 'm, 'f> StatementTranslator<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionBuilder<'ctx, 'm>) -> Self {
        Self {
            parent,
            values: HashMap::new(),
        }
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => self.parent.load_value(name),
        }
    }
}
