#![allow(unused)]

use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::{Context as BackendContext, Context};
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::expressions::Expression;
use crate::function::Scope;
use crate::variable::Variable;

pub struct CompoundStatement {
    pub statements: Vec<Statement>,
}

impl CompoundStatement {
    pub fn compile<'ctx>(
        &self,
        scope: &Scope,
        builder: &Builder<'ctx>,
        function_ir: FunctionValue<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        let entry_block = ctx.append_basic_block(function_ir, "");
        builder.position_at_end(entry_block);

        for statement in self.statements.iter() {
            statement.compile(scope, builder, function_ir, ctx);
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
        scope: &Scope,
        builder: &Builder<'ctx>,
        function_ir: FunctionValue<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        match self {
            Statement::Let(variable) => {
                variable.compile(scope, builder, ctx);
            }
            Statement::Compound(inner) => inner.compile(scope, builder, function_ir, ctx),
            Statement::Return(expression) => {
                let expression = expression.as_ref();
                let return_value = expression.compile(scope, builder, ctx);
                let return_value_ir = return_value.compile_as_basic();
                builder.build_return(Some(&return_value_ir)).unwrap();
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
