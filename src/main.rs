mod ast;
mod backend;
mod basic_block;
mod definition;
mod expression;
mod function;
mod module;
mod scope;
mod statement;
mod types;

use crate::grammar::ModuleParser;
use crate::module::Module;

// function test(x: i8, y: i32, z: i32) {
//     return;
// }

// function test(x: i8, y: i32, z: i32): i32 {
//     return 99;
// }

// const SRC: &'static str = "\
// function sum(x: i8, y: i32, z: i32): i32 {
//     return z;
// }
// ";

// const SRC: &'static str = "\
// function test(x: i64, y: i64, z: i64): i64 {
//     return x + y + z;
// }
// ";

const SRC: &'static str = "\
function test(x: i64, y: i64, z: i64): i64 {
    let a: i64 = 10;
    return x + y + z + a;
}
";

fn main() {
    let parser = ModuleParser::new();
    let module_ast = parser.parse(SRC).unwrap();
    let module = Module::from_ast(module_ast);
    module.traversal_pass();
}

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}
