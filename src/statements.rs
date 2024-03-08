#![allow(unused)]

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::expressions::Expression;

pub struct Scope {
    pub statements: Vec<Statement>,
}

impl Scope {
    pub fn compile<'ctx>(
        &self,
        function_ir: FunctionValue<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        let entry_block = ctx.append_basic_block(function_ir, "");
        builder.position_at_end(entry_block);

        for statement in self.statements.iter() {
            statement.compile(function_ir, builder, ctx);
        }
    }
}

pub enum Statement {
    Evaluation,
    Compound(Scope),
    Return(Box<Expression>),
}

impl Statement {
    pub fn compile<'ctx>(
        &self,
        function_ir: FunctionValue<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        match self {
            Statement::Evaluation => todo!(),
            Statement::Compound(scope) => scope.compile(function_ir, builder, ctx),
            Statement::Return(expression) => {
                let expression = expression.as_ref();
                let return_value = expression.compile(builder, ctx);
                let return_value = TryInto::<BasicValueEnum>::try_into(return_value).unwrap();
                builder.build_return(Some(&return_value)).unwrap();
            }
        }

        // let x = function_ir.get_nth_param(0).unwrap().into_int_value();
        // let y = function_ir.get_nth_param(1).unwrap().into_int_value();
        // let z = function_ir.get_nth_param(2).unwrap().into_int_value();
        // let w = builder
        //     .build_call(fn2, &[], "w")
        //     .unwrap()
        //     .try_as_basic_value()
        //     .unwrap_left()
        //     .into_int_value();

        // let sum = builder.build_int_add(x, y, "sum").unwrap();
        // let sum = builder.build_int_add(sum, z, "sum").unwrap();
        // let sum = builder.build_int_add(sum, w, "sum").unwrap();
    }
}
