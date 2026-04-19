use inkwell::context::Context;

use crate::module_builder::ModuleBuilder;
use crate::parser::grammar::ModuleParser;

mod ast;
mod basic_block;
mod bool_value;
mod constant;
mod definition;
mod errors;
mod expression;
mod expression_translator;
mod float_type;
mod float_value;
mod function;
mod function_translator;
mod function_type;
mod function_value;
mod integer_type;
mod integer_value;
mod module;
mod module_builder;
mod parser;
mod statement;
mod statement_translator;
mod types;
mod value;

const SRC: &'static str = r#"
proc bar(x: f32): f64 {
    return x;
}

proc foo(x: i32): i32 {
    return x;
}

proc test(x: i32, y: i32, z: i32, w: bool): i64 {
    return foo(x);
}
"#;

fn main() {
    let parser = ModuleParser::new();
    let module_ast = parser.parse(SRC).unwrap();

    let context = Context::create();
    let module_builder = ModuleBuilder::new(&context, module_ast).unwrap();
    let module = module_builder.build();
    module.run_test();
}
