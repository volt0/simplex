mod basic_block;
mod expression;
mod function;
mod module;
mod statement;
mod type_spec;
mod value;

use basic_block::BasicBlock;
use expression::{BinaryOperation, Expression, IntegerExpression};
use function::{Function, FunctionArgument};
use inkwell::context::Context;
use module::{Module, ModuleCompiler};
use statement::Statement;
use std::rc::Rc;
use type_spec::TypeSpec;

// mod ast;
//
// mod grammar {
//     include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
// }

type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

fn main() {
    let x = Rc::new(FunctionArgument {
        name: "x".to_string(),
        arg_type: TypeSpec::I64,
        ir_id: Default::default(),
    });
    let y = Rc::new(FunctionArgument {
        name: "y".to_string(),
        arg_type: TypeSpec::I64,
        ir_id: Default::default(),
    });
    let z = Rc::new(FunctionArgument {
        name: "z".to_string(),
        arg_type: TypeSpec::I64,
        ir_id: Default::default(),
    });

    let module = Rc::new(Module {
        functions: vec![Rc::new(Function {
            name: "sum".to_string(),
            args: vec![x.clone(), y.clone(), z.clone()],
            return_type: TypeSpec::I64,
            entry: BasicBlock {
                statements: vec![Statement::Return(Expression::Integer(
                    IntegerExpression::BinaryOperation(
                        BinaryOperation::Add,
                        Box::new(IntegerExpression::BinaryOperation(
                            BinaryOperation::Add,
                            Box::new(IntegerExpression::LoadArgument(x)),
                            Box::new(IntegerExpression::LoadArgument(y)),
                        )),
                        Box::new(IntegerExpression::LoadArgument(z)),
                    ),
                ))],
            },
        })],
    });

    let context = Context::create();
    let module_compiler = ModuleCompiler::new(&context);
    module.compile(&module_compiler);
}
