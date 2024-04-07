use inkwell::context::Context as BackendContext;
use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use crate::parser::parse_module;

mod definitions;
mod errors;
mod expressions;
mod module;
mod parser;
mod scope;
mod statements;
mod types;
mod values;

const SRC: &'static str = "\
fn test(x: byte, y: int, z: int): int {
    let a = 10;
    return x + y + z + a;
}
";

fn main() {
    let module = parse_module(SRC);

    let ctx = BackendContext::create();
    let module_ir = module.compile(&ctx);

    module_ir.print_to_stderr();

    {
        type TestFunc = unsafe extern "C" fn(i8, i32, i32) -> i32;

        let execution_engine = module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        let test_fn: JitFunction<TestFunc> =
            unsafe { execution_engine.get_function("test") }.unwrap();

        let x = -1;
        let y = 2;
        let z = 3;
        let result = unsafe { test_fn.call(x, y, z) };
        dbg!(result);
    }
}
