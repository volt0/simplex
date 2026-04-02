use inkwell::context::Context;

use crate::module_translator::ModuleTranslator;

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
proc test(x: i32, y: i32, z: i32, w: bool): i64 {
    return x + y + z + w;
}
"#;

fn main() {
    let parser = parser::grammar::ModuleParser::new();
    let module = parser.parse(SRC).unwrap();

    let context = Context::create();
    let module_translator = ModuleTranslator::new(&context);
    module.visit(&module_translator).unwrap();
    module_translator.run_test();
}
