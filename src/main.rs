// mod ast;
// mod backend;
// mod basic_block;
// mod definitions;
// mod function;
// mod function_argument;
// mod function_signature;
// mod instruction;
// mod module;
// mod namespace;
// mod scope;
// mod statement;
// mod types;

use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module;
use inkwell::targets::TargetTriple;
use inkwell::OptimizationLevel;
use std::collections::HashMap;

use crate::expression::{BinaryOperation, BinaryOperationExpression, Expression};
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::integer_value::IntegerValue;
use crate::statement_translator::StatementTranslator;
use crate::value::Value;

mod boolean_value;
mod constant;
mod expression;
mod expression_translator;
mod integer_type;
mod integer_value;
mod statement_translator;
mod type_spec;
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

    let statement_translator = StatementTranslator {
        context,
        builder,
        values: HashMap::from([
            (
                "x".to_string(),
                Value::IntegerValue(IntegerValue {
                    ir: function_ir.get_nth_param(0).unwrap().into_int_value(),
                    value_type: IntegerType {
                        is_signed: false,
                        width: IntegerTypeSize::I32,
                    },
                }),
            ),
            (
                "y".to_string(),
                Value::IntegerValue(IntegerValue {
                    ir: function_ir.get_nth_param(1).unwrap().into_int_value(),
                    value_type: IntegerType {
                        is_signed: false,
                        width: IntegerTypeSize::I32,
                    },
                }),
            ),
            (
                "z".to_string(),
                Value::IntegerValue(IntegerValue {
                    ir: function_ir.get_nth_param(2).unwrap().into_int_value(),
                    value_type: IntegerType {
                        is_signed: false,
                        width: IntegerTypeSize::I32,
                    },
                }),
            ),
        ]),
    };

    // let expression = Expression::LoadConstant(Constant::Integer(99));
    // let expression = Expression::LoadValue("x".to_string());
    // let expression = Expression::UnaryOperation(UnaryOperationExpression {
    //     operation: UnaryOperation::Minus,
    //     arg: Box::new(Expression::LoadValue("x".to_string())),
    // });

    let expression = Expression::BinaryOperation(BinaryOperationExpression {
        operation: BinaryOperation::Add,
        lhs: Box::new(Expression::LoadValue("x".to_string())),
        rhs: Box::new(Expression::LoadValue("y".to_string())),
    });
    statement_translator.translate_return_statement(Some(&expression));
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
