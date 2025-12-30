use std::ops::Deref;

use inkwell::values::BasicValueEnum;

use super::expression_translator::ExpressionTranslator;
use super::function_translator::FunctionTranslator;
use crate::basic_block::BasicBlockVisitor;
use crate::expression::Expression;
use crate::statement::{Statement, ValueAssignment};

#[repr(transparent)]
pub struct StatementTranslator<'ctx, 'm, 'f> {
    parent: &'f FunctionTranslator<'ctx, 'm>,
}

impl<'ctx, 'm, 'f> Deref for StatementTranslator<'ctx, 'm, 'f> {
    type Target = FunctionTranslator<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f> BasicBlockVisitor for StatementTranslator<'ctx, 'm, 'f> {
    fn visit_statement(&self, stmt: &Statement) {
        self.translate_statement(stmt);
    }
}

impl<'ctx, 'm, 'f> StatementTranslator<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionTranslator<'ctx, 'm>) -> Self {
        Self { parent }
    }

    fn translate_statement(&self, stmt: &Statement) {
        match stmt {
            Statement::ValueAssignment(var) => {
                self.add_statement_let(var);
            }
            Statement::Return(exp) => {
                self.add_statement_return(exp);
            }
        }
    }

    fn translate_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        ExpressionTranslator::new(self).translate_expression(exp)
    }

    fn add_statement_let(&self, val: &ValueAssignment) {
        let value = self.translate_expression(val.exp.as_ref());
        let value_id = self.store_value(value);
        val.ir_id.set(value_id).unwrap();
    }

    fn add_statement_return(&self, exp: &Expression) {
        let result = self.translate_expression(exp);
        self.builder.build_return(Some(&result)).unwrap();
    }
}
