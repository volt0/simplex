use std::collections::HashMap;
use std::ops::Deref;

use crate::basic_block::{BasicBlock, BasicBlockVisitor};
use crate::errors::CompilationResult;
use crate::expression_translator::ExpressionTranslator;
use crate::function_translator::FunctionTranslator;
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::statement::Statement;
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

impl<'ctx, 'm, 'f> BasicBlockVisitor for StatementTranslator<'ctx, 'm, 'f> {
    fn visit_statement(&self, statement: &Statement) -> CompilationResult<()> {
        let expression_translator = ExpressionTranslator::new(self);
        match statement {
            Statement::BasicBlock(basic_block) => self.translate_basic_block(basic_block),
            Statement::Return(expression) => {
                let return_type = Type::Integer(IntegerType {
                    is_signed: true,
                    width: IntegerTypeSize::I64,
                });

                let value = expression_translator.translate(expression, Some(&return_type))?;
                self.builder().build_return(Some(&value.into_ir()))?;
                Ok(())
            }
        }
    }
}

impl<'ctx, 'm, 'f> StatementTranslator<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionTranslator<'ctx, 'm>) -> Self {
        Self {
            parent,
            values: HashMap::new(),
        }
    }

    pub fn translate_basic_block(&self, basic_block: &BasicBlock) -> CompilationResult<()> {
        basic_block.visit(self)
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.values.get(name) {
            Some(value) => Ok(value.clone()),
            None => self.parent.load_value(name),
        }
    }
}
