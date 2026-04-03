use inkwell::context::Context;

use crate::module_translator::ModuleTranslator;
use crate::parser::grammar::ModuleParser;

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
mod function_value;
mod integer_type;
mod integer_value;
mod module;
mod module_translator;
mod parser;
mod statement;
mod statement_translator;
mod types;
mod value;

const SRC: &'static str = r#"
proc foo(x: i32): i32 {
    return x;
}

proc test(x: i32, y: i32, z: i32, w: bool): i64 {
    return foo(x);
}
"#;

fn main() {
    let parser = ModuleParser::new();
    let module = parser.parse(SRC).unwrap();

    let context = Context::create();
    let mut module_translator = ModuleTranslator::new(&context);
    module.visit(&mut module_translator).unwrap();
    module_translator.run_test();
}
