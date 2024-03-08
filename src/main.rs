use std::rc::Rc;

use definition::{Definition, DefinitionValue};
use inkwell::context::Context as BackendContext;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use crate::expressions::Expression;
use crate::statements::{Scope, Statement};
use function::{Function, FunctionArgument};
use module::Module;

use crate::types::{IntegerType, Type};
use crate::values::Constant;

mod definition;
mod expressions;
mod function;
mod module;
mod statements;
mod types;
mod values;

fn main() {
    let module = Module {
        name: "test".into(),
        defs: vec![Definition {
            name: "test".into(),
            value: DefinitionValue::Function(Rc::new(Function {
                args: vec![
                    Rc::from(FunctionArgument {
                        name: "x".into(),
                        arg_type: Type::SignedInteger(IntegerType::Int),
                    }),
                    Rc::from(FunctionArgument {
                        name: "y".into(),
                        arg_type: Type::SignedInteger(IntegerType::Int),
                    }),
                    Rc::from(FunctionArgument {
                        name: "z".into(),
                        arg_type: Type::SignedInteger(IntegerType::Int),
                    }),
                ],
                return_type: Type::SignedInteger(IntegerType::Int),
                body: Scope {
                    statements: vec![Statement::Return(Box::new(Expression::Constant(
                        Constant::SignedInteger(IntegerType::Int, 99),
                    )))],
                },
            })),
        }],
    };

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
