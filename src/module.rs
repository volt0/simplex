use crate::ast;
use crate::function::{Function, FunctionCompiler};
use crate::types::TypeCompiler;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module as ModuleIR;
use inkwell::targets::TargetTriple;
use inkwell::values::BasicValueEnum;
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};
use std::cell::RefCell;
use std::rc::Rc;

pub struct Module {
    inner: RefCell<ModuleInner>,
}

impl Module {
    pub fn new() -> Rc<Self> {
        Rc::new(Module {
            inner: RefCell::default(),
        })
    }

    pub fn from_ast(module_ast: &ast::Module) -> Rc<Self> {
        let module = Self::new();

        let mut queue = vec![];
        for def in module_ast.definitions.iter() {
            match &def.value {
                ast::DefinitionValue::Function(function_ast) => {
                    let function = Function::from_ast(&function_ast.signature, module.clone());
                    module.add_function(function.clone());

                    if let Some(entry_basic_block) = &function_ast.payload {
                        queue.push((entry_basic_block, function));
                    }
                }
            }
        }

        for (entry_basic_block, function) in queue {
            function.init_implementation(entry_basic_block);
        }

        module
    }
}

#[derive(Default)]
pub struct ModuleInner {
    functions: Vec<Rc<Function>>,
}

impl Module {
    pub fn add_function(&self, function: Rc<Function>) {
        let mut inner = self.inner.borrow_mut();
        inner.functions.push(function);
    }
}

impl<'ctx> Module {
    pub fn compile(&self, module_compiler: &ModuleCompiler<'ctx>) {
        let inner = self.inner.borrow();
        for function in inner.functions.iter().cloned() {
            module_compiler.add_function(&function);
        }
    }
}

pub struct ModuleCompiler<'ctx> {
    context: &'ctx Context,
    ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, BasicValueEnum<'ctx>>>,
}

impl<'ctx> ModuleCompiler<'ctx> {
    pub fn new(context: &'ctx Context) -> ModuleCompiler<'ctx> {
        let ir = context.create_module("test_module");
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        ModuleCompiler {
            context,
            ir,
            values: Default::default(),
        }
    }

    #[inline(always)]
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn store_value(&self, value: BasicValueEnum<'ctx>) -> DefaultKey {
        self.values.borrow_mut().insert(value)
    }

    pub fn load_value(&self, id: DefaultKey) -> Option<BasicValueEnum<'ctx>> {
        self.values.borrow().get(id).cloned()
    }

    pub fn add_function(&self, function: &Function) {
        let type_compiler = TypeCompiler::new(self);
        let function_type = type_compiler.compile_function_type(function);

        let function_ir = self.ir.add_function("sum", function_type, None);
        let function_compiler = FunctionCompiler::new(self, function_ir);
        function.compile(&function_compiler);
    }
}

pub fn compile_module(module: &Module) {
    let context = Context::create();
    let module_compiler = ModuleCompiler::new(&context);
    module.compile(&module_compiler);

    module_compiler.ir.print_to_stderr();

    let execution_engine = module_compiler
        .ir
        .create_jit_execution_engine(OptimizationLevel::None)
        .unwrap();

    unsafe {
        type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

        let sum: JitFunction<SumFunc> = execution_engine.get_function("sum").unwrap();
        let x = 1u64;
        let y = 2u64;
        let z = 3u64;
        dbg!(sum.call(x, y, z));
    }
}
