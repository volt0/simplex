use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;

use crate::expression::Expression;
use crate::expression_translator;
use crate::value::Value;

pub struct StatementTranslator<'ctx> {
    pub context: &'ctx Context,
    pub builder: Builder<'ctx>,
    pub values: HashMap<String, Value<'ctx>>,
}

impl<'ctx> StatementTranslator<'ctx> {
    pub fn translate_return_statement(&self, value: Option<&Expression>) {
        if let Some(value) = value {
            let expression_translator = expression_translator::ExpressionTranslator {
                parent: self,
                values: HashMap::default(),
            };

            let value = expression_translator.translate_expression(value).to_ir();
            self.builder.build_return(Some(&value)).unwrap();
        } else {
            self.builder.build_return(None).unwrap();
        }
    }

    pub fn load_value(&self, name: &str) -> Value<'ctx> {
        self.values.get(name).cloned().unwrap()
    }
}
