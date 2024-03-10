#![allow(unused)]

use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::cache::Cache;
use crate::expressions::Expression;
use crate::variable::Variable;

pub struct CompoundStatement {
    pub statements: Vec<Statement>,
}

impl CompoundStatement {
    pub fn compile<'ctx>(
        &self,
        function_ir: FunctionValue<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
        cache: &mut Cache<'ctx>,
    ) {
        let entry_block = ctx.append_basic_block(function_ir, "");
        builder.position_at_end(entry_block);

        for statement in self.statements.iter() {
            statement.compile(function_ir, builder, ctx, cache);
        }
    }
}

pub enum Statement {
    Let(Rc<Variable>),
    Compound(CompoundStatement),
    Return(Box<Expression>),
}

impl Statement {
    pub fn compile<'ctx>(
        &self,
        function_ir: FunctionValue<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
        cache: &mut Cache<'ctx>,
    ) {
        match self {
            Statement::Let(variable) => variable.compile(builder, ctx, cache),
            Statement::Compound(inner) => inner.compile(function_ir, builder, ctx, cache),
            Statement::Return(expression) => {
                let expression = expression.as_ref();
                let return_value_any = expression.compile(builder, ctx, cache);
                let return_value_basic: BasicValueEnum = return_value_any.try_into().unwrap();
                builder.build_return(Some(&return_value_basic)).unwrap();
            }
        }

        // let w = builder
        //     .build_call(fn2, &[], "w")
        //     .unwrap()
        //     .try_as_basic_value()
        //     .unwrap_left()
        //     .into_int_value();
        // let sum = builder.build_int_add(sum, w, "sum").unwrap();
    }
}
