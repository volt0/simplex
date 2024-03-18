use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use definition::Definition;
use function::{Function, FunctionArgument};
use module::Module;

use crate::expressions::Expression;
use crate::statements::{CompoundStatement, Statement};
use crate::types::{IntegerType, Type};
use crate::variable::Variable;

mod constant;
mod definition;
mod expressions;
mod function;
mod module;
mod scope;
mod statements;
mod types;
mod values;
mod variable;

fn main() {
    let module = Module::new(
        "test".into(),
        vec![{
            Definition::define_function(
                "test".into(),
                Function::new(
                    vec![
                        FunctionArgument::new("x".into(), Type::SignedInteger(IntegerType::Int)),
                        FunctionArgument::new("y".into(), Type::SignedInteger(IntegerType::Int)),
                        FunctionArgument::new("z".into(), Type::SignedInteger(IntegerType::Int)),
                    ],
                    Type::SignedInteger(IntegerType::Int),
                    CompoundStatement {
                        statements: vec![
                            Statement::Let(Rc::new(Variable {
                                name: "a".into(),
                                value_type: Type::SignedInteger(IntegerType::Int),
                                value_init: Box::new(Expression::Identifier("x".into())),
                            })),
                            Statement::Return(
                                // Expression::new_int_const(99),
                                Box::new(Expression::Identifier("a".into())),
                                // Expression::new_add(
                                //     Expression::new_int_const(99),
                                //     Box::new(Expression::Identifier(Identifier("x".into()))),
                                // ),
                            ),
                        ],
                    },
                ),
            )
        }],
    );

    let ctx = BackendContext::create();
    let module_ir = module.compile(&ctx);

    module_ir.print_to_stderr();

    {
        type TestFunc = unsafe extern "C" fn(u64, u64, u64) -> i64;

        let execution_engine = module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let test_fn: JitFunction<TestFunc> =
            unsafe { execution_engine.get_function("test") }.unwrap();

        let x = 1;
        let y = 2;
        let z = 3;
        let result = unsafe { test_fn.call(x, y, z) };
        dbg!(result);
    }
}
