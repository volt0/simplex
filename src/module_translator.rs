use std::collections::HashMap;
use std::ops::Deref;

use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::targets::TargetTriple;
use inkwell::OptimizationLevel;

use crate::errors::{CompilationError, CompilationResult};
use crate::function::Function;
use crate::function_translator::FunctionTranslator;
use crate::function_type::FunctionType;
use crate::function_value::FunctionValue;
use crate::module::ModuleVisitor;
use crate::translator::Translator;
use crate::value::Value;

type ModuleIR<'ctx> = inkwell::module::Module<'ctx>;

pub struct ModuleTranslator<'ctx> {
    parent: &'ctx Translator,
    module_ir: ModuleIR<'ctx>,
    globals: HashMap<String, Value<'ctx>>,
}

impl<'ctx> Deref for ModuleTranslator<'ctx> {
    type Target = Translator;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx> ModuleVisitor for ModuleTranslator<'ctx> {
    fn visit_function(&mut self, name: Option<&str>, func: &Function) -> CompilationResult<()> {
        let func_signature = func.signature();
        let func_type = FunctionType::from_ast(self.context(), func_signature)?;
        let func_type_ir = func_type.ir().clone();
        let func_ir = self
            .module_ir
            .add_function(name.unwrap_or(""), func_type_ir, None);

        if let Some(name) = name {
            self.globals.insert(
                name.to_string(),
                FunctionValue::new(func_ir, func_type).into(),
            );
        }

        let func_translator = FunctionTranslator::new(func_ir, func_signature, self)?;
        func.visit(&func_translator)?;

        Ok(())
    }
}

impl<'ctx> ModuleTranslator<'ctx> {
    pub fn new(parent: &'ctx Translator) -> ModuleTranslator<'ctx> {
        let module_ir = parent.context().create_module("test_module");
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        ModuleTranslator {
            parent,
            module_ir,
            globals: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn context(&self) -> &'ctx Context {
        self.parent.context()
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.globals.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(CompilationError::UnresolvedName(name.to_string())),
        }
    }

    pub fn run_test(&self) {
        self.module_ir.print_to_stderr();

        type TestFunc = unsafe extern "C" fn(i32, i32, i32, bool) -> i64;

        let execution_engine = self
            .module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let test_func: JitFunction<'_, TestFunc> =
                execution_engine.get_function("test").unwrap();

            let x = 1i32;
            let y = 2i32;
            let z = 3i32;
            let w = true;
            dbg!(test_func.call(x, y, z, w));
        }
    }
}
