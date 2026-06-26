use inkwell::context::Context;

use crate::parser::grammar::ModuleParser;
use crate::target_builder::TargetBuilder;

mod ast;
mod block;
mod constant;
mod definition;
mod errors;
mod expression;
mod function;
mod module;
mod parser;
mod statement;
mod target_builder;
mod types;
mod values;

const SRC: &'static str = r#"
proc bar(x: f32): f64 {
    return x;
}

proc foo(x: i32): i32 {
    return x;
}

proc test(x: u8, y: i16, z: i32, w: bool): i64 {
    return foo(x);
}
"#;

fn main() {
    let parser = ModuleParser::new();
    let module_ast = parser.parse(SRC).unwrap();

    let context = Context::create();
    let target_builder = TargetBuilder::new(&context);
    let module = target_builder
        .create_module("test_module", module_ast)
        .unwrap();
    module.run_test();
}
