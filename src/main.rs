use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::OptimizationLevel;

fn main() {
    let context = Context::create();
    let module_ir = context.create_module("test_module");
    module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

    compile_function(&context, &module_ir);

    module_ir.print_to_stderr();
    run_test(&module_ir);
}

pub fn compile_function<'ctx>(context: &'ctx Context, module_ir: &Module<'ctx>) {
    let fn_type = context.i32_type().fn_type(
        &[
            context.i32_type().into(),
            context.i32_type().into(),
            context.i32_type().into(),
        ],
        false,
    );

    let function_ir = module_ir.add_function("test", fn_type, None);
    let basic_block = context.append_basic_block(function_ir.clone(), "entry");

    let builder = context.create_builder();
    builder.position_at_end(basic_block);

    let x = function_ir.get_nth_param(0).unwrap().into_int_value();
    let y = function_ir.get_nth_param(1).unwrap().into_int_value();
    let z = function_ir.get_nth_param(2).unwrap().into_int_value();

    let sum = builder.build_int_add(x, y, "sum").unwrap();
    let sum = builder.build_int_add(sum, z, "sum").unwrap();

    builder.build_return(Some(&sum)).unwrap();
}

fn run_test(module_ir: &Module) {
    type TestFunc = unsafe extern "C" fn(i32, i32, i32) -> i32;

    let execution_engine = module_ir
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        let test_function: JitFunction<'_, TestFunc> =
            execution_engine.get_function("test").unwrap();

        let x = 1i32;
        let y = 2i32;
        let z = 3i32;
        dbg!(test_function.call(x, y, z));
    }
}
