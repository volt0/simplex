use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::errors::{CompilationError, CompilationResult};
use crate::expression_translator::ExpressionTranslator;
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::statement::Statement;
use crate::types::Type;
use crate::value::Value;

pub struct StatementTranslator<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub values: HashMap<String, Value<'ctx>>,
}

impl<'ctx> StatementTranslator<'ctx> {
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn translate(&self, statement: &Statement) -> CompilationResult<()> {
        let expression_translator = ExpressionTranslator::new(self);
        match statement {
            Statement::Return(expression) => {
                let return_type = Type::Integer(IntegerType {
                    is_signed: true,
                    width: IntegerTypeSize::I64,
                });

                let value = expression_translator.translate(expression, Some(&return_type))?;
                self.builder.build_return(Some(&value.into_ir()))?;
            }
        }
        Ok(())
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        self.values
            .get(name)
            .ok_or(CompilationError::UnresolvedName(name.to_string()))
            .cloned()
    }
}
