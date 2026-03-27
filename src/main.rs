use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::OptimizationLevel;

use crate::expression::{BinaryOperation, BinaryOperationExpression, Expression};

mod expression;
mod value;

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

    let expression = Expression::BinaryOperation(BinaryOperationExpression {
        operation: BinaryOperation::Add,
        // lhs: Box::new(Expression::LoadValue("x".to_string())),
        // rhs: Box::new(Expression::LoadValue("y".to_string())),
        lhs: Box::new(Expression::LoadConstant(expression::Constant::Integer(100))),
        rhs: Box::new(Expression::LoadConstant(expression::Constant::Integer(99))),
    });

    let expression_translator = expression::ExpressionTranslator { context, builder };
    let value = expression_translator.translate(&expression);

    let builder = expression_translator.builder;
    let value_ir = value.into_ir();
    builder.build_return(Some(&value_ir)).unwrap();
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
