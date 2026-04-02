use std::collections::HashMap;
use std::ops::Deref;

use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::expression::Expression;
use crate::expression_translator::ExpressionTranslator;
use crate::function_translator::FunctionTranslator;
use crate::integer_type::{IntegerType, IntegerTypeSize};
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

    fn add_return_statement(&self, expression: &Expression) -> CompilationResult<()> {
        let expression_translator = ExpressionTranslator::new(self);
        let return_type = Type::Integer(IntegerType {
            is_signed: true,
            width: IntegerTypeSize::I64,
        });

        self.builder().build_return(Some(
            &expression_translator
                .translate_expression(expression, Some(&return_type))?
                .into_ir(),
        ))?;

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
