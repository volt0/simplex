use inkwell::context::Context as BackendContext;

use module::Module;
use scope::Scope;

mod ast;
mod function;
mod module;
mod scope;
mod types;

// function test(x: i8, y: i32, z: i32) {
//     return;
// }

// function test(x: i8, y: i32, z: i32): i32 {
//     return 99;
// }

const SRC: &'static str = "\
function test(x: i8, y: i32, z: i32): i32 {
    return z;
}
";

// const SRC: &'static str = "\
// function test(x: i8, y: i32, z: i32): i32 {
//     let a = 10;
//     return x + y + z + a;
// }
// ";

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub fn main() {
    let parser = grammar::ModuleParser::new();
    let module_ast = parser.parse(SRC).unwrap();

    let ctx = BackendContext::create();
    let root_scope = Scope::default();
    let module = Module::compile("foo", module_ast, &root_scope, &ctx);
    module.print_to_stderr();

    // {
    //     type TestFunc = unsafe extern "C" fn(i8, i32, i32) -> i32;
    //
    //     let execution_engine = module
    //         .ir
    //         .create_jit_execution_engine(OptimizationLevel::None)
    //         .unwrap();
    //
    //     let test_fn: JitFunction<TestFunc> =
    //         unsafe { execution_engine.get_function("test") }.unwrap();
    //
    //     let x = -1;
    //     let y = 2;
    //     let z = 3;
    //     let result = unsafe { test_fn.call(x, y, z) };
    //     dbg!(result);
    // }
}
