use crate::function::{Function, FunctionCompiler};
use crate::value::Value;
use crate::SumFunc;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module as ModuleIR;
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Module {
    pub functions: Vec<Rc<Function>>,
}

impl Module {
    pub fn compile(&self, builder: &ModuleCompiler) {
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

pub struct ModuleCompiler<'ctx> {
    context: &'ctx Context,
    ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, Value<'ctx>>>,
}

impl<'ctx> ModuleCompiler<'ctx> {
    pub fn new(context: &'ctx Context) -> Self {
        let module_ir = context.create_module("sum");
        ModuleCompiler {
            context,
            ir: module_ir,
            values: Default::default(),
        }
    }

    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn store_value(&self, value: Value<'ctx>) -> DefaultKey {
        self.values.borrow_mut().insert(value)
    }

    pub fn load_value(&self, id: DefaultKey) -> Option<Value<'ctx>> {
        self.values.borrow().get(id).cloned()
    }

    pub fn add_function(&self, function: &Function) -> FunctionCompiler<'ctx, '_> {
        let fn_type = function.compile_type(self);
        let function_ir = self.ir.add_function("sum", fn_type, None);

        FunctionCompiler {
            context: &self.context,
            module_compiler: self,
            ir: function_ir,
        }
    }
}
