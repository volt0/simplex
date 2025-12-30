use inkwell::context::Context;

use crate::backend::ModuleTranslator;
use crate::grammar::ModuleParser;
use crate::module::Module;

mod ast;
mod backend;
mod basic_block;
mod constant;
mod definitions;
mod expression;
mod function;
mod function_argument;
mod function_signature;
mod instruction;
mod module;
mod namespace;
mod scope;
mod statement;
mod types;

// proc test(x: i8, y: i32, z: i32) {
//     return;
// }

// proc test(x: i8, y: i32, z: i32): i32 {
//     return 99;
// }

// proc test(x: i8, y: i32, z: i32): i32 {
//     return z;
// }

// proc test(x: i64, y: i64, z: i64): i64 {
//     return x + y + z;
// }

const SRC: &'static str = "\
proc test(x: i64, y: i64, z: i64): i64 {
    let a: i64 = 10;
    return x + y + z + a;
}
";

fn main() {
    let parser = ModuleParser::new();
    let module_ast = parser.parse(SRC).unwrap();
    let module = Module::new(module_ast);

    let context = Context::create();
    let module_translator = ModuleTranslator::new(&context);
    module.visit(&module_translator);

    module_translator.dbg_print();

    unsafe {
        type TestFunction = unsafe extern "C" fn(u64, u64, u64) -> u64;

        let execution_engine = module_translator.create_execution_engine();
        let test_function = execution_engine
            .get_function::<TestFunction>("test")
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
