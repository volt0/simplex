use crate::module::Module;

mod grammar {
    include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
}

pub fn parse_module(source: &str) -> Module {
    let parser = grammar::ModuleParser::new();
    parser.parse(source).unwrap()
}
