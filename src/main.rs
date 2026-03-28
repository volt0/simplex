use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::OptimizationLevel;

use crate::expression::{BinaryOperation, BinaryOperationExpression, Expression};
use crate::value::{IntegerType, IntegerTypeSize, IntegerValue, Value};

mod errors;
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
    let fn_type = context.i64_type().fn_type(
        &[
            context.i16_type().into(),
            context.i32_type().into(),
            context.i64_type().into(),
        ],
        false,
    );

    let function_ir = module_ir.add_function("test", fn_type, None);
    let basic_block = context.append_basic_block(function_ir.clone(), "entry");

    let builder = context.create_builder();
    builder.position_at_end(basic_block);

    let values = HashMap::from([
        (
            "x".to_string(),
            Value::Integer(IntegerValue {
                ir: function_ir.get_nth_param(0).unwrap().into_int_value(),
                value_type: IntegerType {
                    is_signed: true,
                    width: IntegerTypeSize::I16,
                },
            }),
        ),
        (
            "y".to_string(),
            Value::Integer(IntegerValue {
                ir: function_ir.get_nth_param(1).unwrap().into_int_value(),
                value_type: IntegerType {
                    is_signed: true,
                    width: IntegerTypeSize::I32,
                },
            }),
        ),
        (
            "z".to_string(),
            Value::Integer(IntegerValue {
                ir: function_ir.get_nth_param(2).unwrap().into_int_value(),
                value_type: IntegerType {
                    is_signed: true,
                    width: IntegerTypeSize::I64,
                },
            }),
        ),
    ]);

    let expression = Expression::BinaryOperation(BinaryOperationExpression {
        operation: BinaryOperation::Add,
        lhs: Box::new(Expression::BinaryOperation(BinaryOperationExpression {
            operation: BinaryOperation::Add,
            lhs: Box::new(Expression::BinaryOperation(BinaryOperationExpression {
                operation: BinaryOperation::Add,
                lhs: Box::new(Expression::LoadValue("x".to_string())),
                rhs: Box::new(Expression::LoadValue("y".to_string())),
            })),
            rhs: Box::new(Expression::LoadValue("z".to_string())),
        })),
        rhs: Box::new(Expression::LoadConstant(expression::Constant::Integer(100))),
    });

    let expression_translator = expression::ExpressionTranslator {
        context,
        builder,
        values,
    };
    let value = expression_translator.translate(&expression).unwrap();

    let builder = expression_translator.builder;
    let value_ir = value.into_ir();
    builder.build_return(Some(&value_ir)).unwrap();
}

fn run_test(module_ir: &Module) {
    type TestFunc = unsafe extern "C" fn(i16, i32, i64) -> i64;

    let execution_engine = module_ir
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        let test_function: JitFunction<'_, TestFunc> =
            execution_engine.get_function("test").unwrap();

        let x = 1i16;
        let y = 2i32;
        let z = 3i64;
        dbg!(test_function.call(x, y, z));
    }
}
