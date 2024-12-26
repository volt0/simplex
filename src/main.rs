use inkwell::basic_block::BasicBlock as BasicBlockIR;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module as ModuleIR;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicValue, BasicValueEnum, FunctionValue, IntValue};
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};
use std::cell::{OnceCell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

// mod ast;
//
// mod grammar {
//     include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
// }

type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

struct ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    builder: &'b Builder<'ctx>,
    basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 'b> ExpressionCompiler<'ctx, 'm, 'f, 'b>
where
    'm: 'ctx,
    'f: 'm,
    'b: 'f,
{
    fn compile_integer_load_argument(&self, arg: Rc<FunctionArgument>) -> IntValue<'ctx> {
        if let Value::Integer(arg_ir) = self.get_value(arg.ir_id.get().unwrap().clone()).unwrap() {
            return arg_ir;
        }
        panic!()
    }

    fn compile_integer_expression(&self, exp: &IntegerExpression) -> IntValue<'ctx> {
        dbg!();
        match exp {
            IntegerExpression::BinaryOperation(_, _, _) => todo!(),
            IntegerExpression::LoadArgument(arg) => {
                self.compile_integer_load_argument(arg.clone())
                // self.get_value(arg.as_ref().ir_id.get().unwrap().clone()).unwrap()
            }
        }
    }

    fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        dbg!();
        match exp {
            Expression::Integer(exp) => self.compile_integer_expression(exp).as_basic_value_enum(),
        }
    }
}

impl<'ctx, 'm, 'f, 'b> Deref for ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    type Target = BasicBlockCompiler<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.basic_block_compiler
    }
}

struct BasicBlockCompiler<'ctx, 'm, 'f> {
    context: &'ctx Context,
    function_compiler: &'f FunctionCompiler<'ctx, 'm>,
    builder: Builder<'ctx>,
    basic_block: BasicBlockIR<'ctx>,
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        dbg!();
        let exp_compiler = ExpressionCompiler {
            builder: &self.builder,
            basic_block_compiler: self,
        };
        exp.compile(&exp_compiler)
    }

    fn compile_statement_return(&self, exp: &Expression) {
        let result = self.compile_expression(exp);
        self.builder.build_return(Some(&result)).unwrap();

        // let x = self.ir.get_nth_param(0).unwrap().into_int_value();
        // let y = self.ir.get_nth_param(1).unwrap().into_int_value();
        // let z = self.ir.get_nth_param(2).unwrap().into_int_value();
        //
        // let sum = self.builder.build_int_add(x, y, "sum").unwrap();
        // let sum = self.builder.build_int_add(sum, z, "sum").unwrap();
        //
        // self.builder.build_return(Some(&sum)).unwrap();
    }
}

impl<'ctx, 'm, 'f> Deref for BasicBlockCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.function_compiler
    }
}

struct FunctionCompiler<'ctx, 'm> {
    context: &'ctx Context,
    module_compiler: &'m ModuleCompiler<'ctx>,
    ir: FunctionValue<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionCompiler<'ctx, 'm> {
    type Target = ModuleCompiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.module_compiler
    }
}

impl<'ctx, 'm> FunctionCompiler<'ctx, 'm> {
    fn add_basic_block(&self) -> BasicBlockCompiler<'ctx, 'm, '_> {
        let builder = self.context.create_builder();
        let basic_block = self.context.append_basic_block(self.ir, "entry");
        builder.position_at_end(basic_block);

        BasicBlockCompiler {
            context: self.context,
            function_compiler: self,
            builder,
            basic_block,
        }
    }
}

struct ModuleCompiler<'ctx> {
    context: &'ctx Context,
    ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, Value<'ctx>>>,
}

impl<'ctx> ModuleCompiler<'ctx> {
    fn add_value(&self, value: Value<'ctx>) -> DefaultKey {
        self.values.borrow_mut().insert(value)
    }

    fn get_value(&self, id: DefaultKey) -> Option<Value<'ctx>> {
        self.values.borrow().get(id).cloned()
    }

    fn add_function(&self, function: &Function) -> FunctionCompiler<'ctx, '_> {
        let fn_type = function.compile_type(self);
        let function_ir = self.ir.add_function("sum", fn_type, None);

        FunctionCompiler {
            context: &self.context,
            module_compiler: self,
            ir: function_ir,
        }
    }
}

#[derive(Clone)]
enum Value<'ctx> {
    Integer(IntValue<'ctx>),
}

#[derive(Clone)]
enum TypeSpec {
    I64,
}

impl TypeSpec {
    fn into_ir(self, context: &Context) -> BasicTypeEnum {
        match self {
            TypeSpec::I64 => context.i64_type().as_basic_type_enum(),
        }
    }
}

enum BinaryOperation {
    Add,
}

enum IntegerExpression {
    BinaryOperation(
        BinaryOperation,
        Box<IntegerExpression>,
        Box<IntegerExpression>,
    ),
    LoadArgument(Rc<FunctionArgument>),
}

impl<'ctx, 'm, 'f, 'b> IntegerExpression {
    fn compile(&self, compiler: &ExpressionCompiler<'ctx, 'm, 'f, 'b>) -> IntValue<'ctx> {
        match self {
            IntegerExpression::BinaryOperation(_, _, _) => {
                todo!()
            }
            IntegerExpression::LoadArgument(arg) => {
                todo!()
                // compiler.compile_integer_load_argument(arg.clone())
                // compiler.compile_integer_expression(exp)
            }
        }
    }
}

enum Expression {
    Integer(IntegerExpression),
}

impl Expression {
    fn compile<'ctx>(&self, compiler: &ExpressionCompiler) -> BasicValueEnum<'ctx> {
        todo!()
        // match self {
        //     Expression::Integer(exp) => compiler
        //         .compile_integer_expression(exp)
        //         .as_basic_value_enum(),
        // }
    }
}

enum Statement {
    Return(Expression),
}

struct BasicBlock {
    statements: Vec<Statement>,
}

impl BasicBlock {
    fn compile(&self, compiler: &BasicBlockCompiler) {
        for statement in &self.statements {
            match statement {
                Statement::Return(exp) => compiler.compile_statement_return(exp),
            }
        }

        dbg!();
    }
}

struct FunctionArgument {
    name: String,
    arg_type: TypeSpec,
    ir_id: OnceCell<DefaultKey>,
}

struct Function {
    name: String,
    args: Vec<Rc<FunctionArgument>>,
    return_type: TypeSpec,
    entry: BasicBlock,
}

impl Function {
    fn compile(&self, compiler: &FunctionCompiler) {
        for (pos_id, arg) in self.args.iter().enumerate() {
            let arg_ir = compiler.ir.get_nth_param(pos_id as u32).unwrap();
            let arg_value = match arg.arg_type {
                TypeSpec::I64 => Value::Integer(arg_ir.into_int_value()),
            };
            let arg_ir_id = compiler.add_value(arg_value);
            arg.ir_id.set(arg_ir_id).unwrap();
        }
        let basic_block_compiler = compiler.add_basic_block();
        self.entry.compile(&basic_block_compiler)
    }

    fn compile_type<'ctx>(&self, builder: &ModuleCompiler<'ctx>) -> FunctionType<'ctx> {
        let return_type_ir = self.return_type.clone().into_ir(&builder.context);

        let arg_type_irs: Vec<BasicMetadataTypeEnum> = self
            .args
            .iter()
            .map(|arg| arg.arg_type.clone().into_ir(&builder.context).into())
            .collect();

        return_type_ir.fn_type(&arg_type_irs, false)
    }
}

struct Module {
    functions: Vec<Rc<Function>>,
}

impl Module {
    fn compile(&self, builder: &ModuleCompiler) {
        for function in self.functions.iter().cloned() {
            let builder = builder.add_function(function.as_ref());
            function.compile(&builder)
        }

        let execution_engine = builder
            .ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let sum: JitFunction<SumFunc> = execution_engine.get_function("sum").unwrap();
            let x = 1u64;
            let y = 2u64;
            let z = 3u64;
            dbg!(sum.call(x, y, z));
        }
    }
}

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
    let module_ir = context.create_module("sum");
    let module_builder = ModuleCompiler {
        context: &context,
        ir: module_ir,
        values: Default::default(),
    };
    module.compile(&module_builder);
}
