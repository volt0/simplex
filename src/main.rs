use inkwell::context::Context as BackendContext;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use definition::Definition;
use function::{Function, FunctionArgument};
use module::Module;

use crate::expressions::Expression;
use crate::statements::{Scope, Statement};
use crate::types::{IntegerType, Type};

mod definition;
mod expressions;
mod function;
mod module;
mod statements;
mod types;
mod values;

fn main() {
    let module = Module::new(
        "test".into(),
        vec![{
            let x = FunctionArgument::new("x".into(), Type::SignedInteger(IntegerType::Int));
            let y = FunctionArgument::new("y".into(), Type::SignedInteger(IntegerType::Int));
            let z = FunctionArgument::new("z".into(), Type::SignedInteger(IntegerType::Int));

            Definition::define_function(
                "test".into(),
                Function::new(
                    vec![x.clone(), y, z],
                    Type::SignedInteger(IntegerType::Int),
                    Scope {
                        statements: vec![Statement::Return(
                            Expression::new_int_const(99),
                            // Expression::new_add(
                            //     Expression::new_int_const(99),
                            //     Box::new(Expression::Identifier(Identifier {
                            //         name: "x".into(),
                            //         resolved: OnceCell::from(Value::Argument(x)),
                            //     })),
                            // ),
                        )],
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
