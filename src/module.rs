use std::collections::HashMap;
use std::ops::Deref;

use inkwell::execution_engine::JitFunction;
use inkwell::OptimizationLevel;

use crate::ast;
use crate::definition::Definition;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::{Function, FunctionBuilder};
use crate::statement::Block;
use crate::target::TargetBuilder;
use crate::types::TypeTranslator;
use crate::values::Value;

type ModuleIR<'ctx> = inkwell::module::Module<'ctx>;

pub struct Module<'ctx> {
    pub module_ir: ModuleIR<'ctx>,
    pub definitions: HashMap<String, Definition<'ctx>>,
}

impl<'ctx> Module<'ctx> {
    pub fn new(module_ir: ModuleIR<'ctx>) -> Self {
        Module {
            module_ir,
            definitions: HashMap::new(),
        }
    }

    pub fn run_test(&self) {
        self.module_ir.print_to_stderr();

        type TestFunc = unsafe extern "C" fn(u8, i16, i32, bool) -> i64;

        let execution_engine = self
            .module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let test_func: JitFunction<'_, TestFunc> =
                execution_engine.get_function("test").unwrap();

            let x = 1u8;
            let y = 2i16;
            let z = 3i32;
            let w = true;
            dbg!(test_func.call(x, y, z, w));
        }
    }
}

pub struct ModuleBuilder<'ctx> {
    module: Module<'ctx>,
    target_builder: &'ctx TargetBuilder<'ctx>,
}

impl<'ctx> ModuleBuilder<'ctx> {
    pub fn new(module: Module<'ctx>, target_builder: &'ctx TargetBuilder<'ctx>) -> Self {
        Self {
            module,
            target_builder,
        }
    }

    pub fn add_definition(&mut self, def_ast: ast::Definition) -> CompilationResult<()> {
        let name = def_ast.name.clone();
        let def = Definition::new_from_ast(def_ast, self)?;
        self.module.definitions.insert(name, def);
        Ok(())
    }

    pub fn create_function(
        &mut self,
        name: &str,
        signature: ast::FunctionSignature,
        body: Block,
    ) -> CompilationResult<Function<'ctx>> {
        let type_translator = TypeTranslator::new(self);
        let func_type = type_translator.create_function_type(&signature)?;
        let func_type_ir = func_type.ir().clone();
        let func_ir = self.module_ir().add_function(name, func_type_ir, None);

        let func = Function::new(func_ir, func_type)?;
        let func_builder = FunctionBuilder::new(func, self)?;
        func_builder.attach_body(body)?;

        Ok(func_builder.build())
    }

    #[inline(always)]
    pub fn module_ir(&self) -> &ModuleIR<'ctx> {
        &self.module.module_ir
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.module.definitions.get(name) {
            Some(def) => Ok(match def {
                Definition::Function(func) => func.clone().into(),
            }),
            None => Err(CompilationError::UnresolvedName(name.to_string())),
        }
    }

    pub fn build(self) -> Module<'ctx> {
        self.module
    }
}

impl<'ctx> Deref for ModuleBuilder<'ctx> {
    type Target = TargetBuilder<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.target_builder
    }
}
