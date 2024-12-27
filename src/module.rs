use crate::function::{compile_function, Function};
use crate::value::Value;
use crate::SumFunc;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module as ModuleIR;
use inkwell::values::FunctionValue;
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Module {
    pub functions: Vec<Rc<Function>>,
}

impl Module {}

pub struct ModuleCompiler<'ctx> {
    context: &'ctx Context,
    ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, Value<'ctx>>>,
}

impl<'ctx> ModuleCompiler<'ctx> {
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn store_value(&self, value: Value<'ctx>) -> DefaultKey {
        self.values.borrow_mut().insert(value)
    }

    pub fn load_value(&self, id: DefaultKey) -> Option<Value<'ctx>> {
        self.values.borrow().get(id).cloned()
    }

    pub fn add_function(&self, function: &Function) -> FunctionValue<'ctx> {
        let fn_type = function.compile_type(self);
        self.ir.add_function("sum", fn_type, None)
    }
}

pub fn compile_module(module: Rc<Module>) {
    let context = Context::create();
    let module_ir = context.create_module("sum");
    let module_compiler = ModuleCompiler {
        context: &context,
        ir: module_ir,
        values: Default::default(),
    };

    for function in module.functions.iter().cloned() {
        compile_function(function, &module_compiler);
    }

    let execution_engine = module_compiler
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
