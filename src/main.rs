use crate::parser::grammar::ModuleParser;
use crate::translator::Translator;

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
mod translator;
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
    let module = parser.parse(SRC).unwrap();

    let translator = Translator::new();
    translator.translate_module(&module);
}
