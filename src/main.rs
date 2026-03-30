use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::OptimizationLevel;

use crate::function_translator::FunctionTranslator;

mod basic_block;
mod bool_value;
mod constant;
mod errors;
mod expression;
mod expression_translator;
mod float_type;
mod float_value;
mod function;
mod function_translator;
mod integer_type;
mod integer_value;
mod parser;
mod statement;
mod statement_translator;
mod types;
mod value;

const SRC: &'static str = r#"
proc test(x: i32, y: i32, z: i32, w: bool): i64 {
    return x + y + z;
}
"#;

fn main() {
    let context = Context::create();
    let module_ir = context.create_module("test_module");
    module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

    compile_function(&context, &module_ir);

    module_ir.print_to_stderr();
    run_test(&module_ir);
}

pub fn compile_function<'ctx>(context: &'ctx Context, module_ir: &Module<'ctx>) {
    let fn_type = context.i64_type().fn_type(
        &[
            context.i32_type().into(),
            context.i32_type().into(),
            context.i32_type().into(),
            context.bool_type().into(),
        ],
        false,
    );
    let function_ir = module_ir.add_function("test", fn_type, None);

    let parser = parser::grammar::FunctionParser::new();
    let function = parser.parse(SRC).unwrap();
    let function_translator = FunctionTranslator::new(function_ir, &function, context).unwrap();
    function.visit(&function_translator).unwrap();
}

fn run_test(module_ir: &Module) {
    type TestFunc = unsafe extern "C" fn(i32, i32, i32, bool) -> i64;

    let execution_engine = module_ir
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        let test_function: JitFunction<'_, TestFunc> =
            execution_engine.get_function("test").unwrap();

        let x = 1i32;
        let y = 2i32;
        let z = 3i32;
        let w = true;
        dbg!(test_function.call(x, y, z, w));
    }
}
