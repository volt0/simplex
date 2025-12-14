use inkwell::context::Context;
use inkwell::OptimizationLevel;

use crate::grammar::ModuleParser;
use crate::module::{Module, ModuleTranslator};

mod ast;
mod basic_block;
mod definitions;
mod expression;
mod module;
mod scope;
mod statement;
mod types;

// function test(x: i8, y: i32, z: i32) {
//     return;
// }

// function test(x: i8, y: i32, z: i32): i32 {
//     return 99;
// }

// function sum(x: i8, y: i32, z: i32): i32 {
//     return z;
// }

// function test(x: i64, y: i64, z: i64): i64 {
//     return x + y + z;
// }

// function test(x: i64, y: i64, z: i64): i64 {
//     let a: i64 = 10;
//     return x + y + z + a;
// }

const SRC: &'static str = "\
function test(x: i32, y: i32, z: i32): i32 {
    return 99;
}
";

fn main() {
    let parser = ModuleParser::new();
    let module_ast = parser.parse(SRC).unwrap();
    let module = Module::from_ast(module_ast);
    module.traversal_pass();

    let context = Context::create();
    let module_translator = ModuleTranslator::new(&context);
    module.visit(&module_translator);

    module_translator.module_ir.print_to_stderr();

    let execution_engine = module_translator
        .module_ir
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        type TestFunction = unsafe extern "C" fn(u64, u64, u64) -> u64;

        let test_function = execution_engine
            .get_function::<TestFunction>("sum")
            .unwrap();

        let x = 1u64;
        let y = 2u64;
        let z = 3u64;
        dbg!(test_function.call(x, y, z));
    }
}

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}
