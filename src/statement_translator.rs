use std::collections::HashMap;
use std::ops::Deref;

use inkwell::values::BasicValueEnum;

use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::expression::Expression;
use crate::expression_translator::ExpressionTranslator;
use crate::function_translator::FunctionTranslator;
use crate::statement::StatementVisitor;
use crate::types::Type;
use crate::value::Value;

pub struct StatementTranslator<'ctx, 'm, 'f> {
    parent: &'f FunctionTranslator<'ctx, 'm>,
    values: HashMap<String, Value<'ctx>>,
}

impl<'ctx, 'm, 'f> Deref for StatementTranslator<'ctx, 'm, 'f> {
    type Target = FunctionTranslator<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f> StatementVisitor for StatementTranslator<'ctx, 'm, 'f> {
    fn visit_basic_block(&self, block: &BasicBlock) -> CompilationResult<()> {
        block.visit(self)
    }

    fn add_return_statement(&self, expr: &Expression) -> CompilationResult<()> {
        let func_signature = self.function_signature();
        let expr_translator = ExpressionTranslator::new(self);
        let expr_type = Type::new(&func_signature.return_type, self.context());
        let expr_ir: BasicValueEnum<'ctx> = expr_translator
            .translate_expression(expr, Some(expr_type))?
            .try_into()?;

        self.builder().build_return(Some(&expr_ir))?;
        Ok(())
    }
}

impl<'ctx, 'm, 'f> StatementTranslator<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionTranslator<'ctx, 'm>) -> Self {
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
