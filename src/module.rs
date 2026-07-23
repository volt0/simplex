use std::collections::HashMap;
use std::ops::Deref;

use crate::ast;
use crate::block::Block;
use crate::definition::Definition;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::{Function, FunctionBuilder};
use crate::target_builder::TargetBuilder;
use crate::values::Value;
use inkwell::execution_engine::JitFunction;
use inkwell::targets::TargetTriple;
use inkwell::OptimizationLevel;

type ModuleIR<'ctx> = inkwell::module::Module<'ctx>;

pub struct Module<'ctx> {
    pub(crate) module_ir: ModuleIR<'ctx>,
    pub defs: HashMap<String, Definition<'ctx>>,
}

impl<'ctx> Module<'ctx> {
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
    parent: &'ctx TargetBuilder<'ctx>,
    module: Module<'ctx>,
}

impl<'ctx> ModuleBuilder<'ctx> {
    pub fn new(parent: &'ctx TargetBuilder<'ctx>, name: &str) -> Self {
        let module_ir = parent.context().create_module(name);
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        Self {
            parent,
            module: Module {
                module_ir,
                defs: HashMap::new(),
            },
        }
    }

    pub fn define(&mut self, def_ast: ast::Definition) -> CompilationResult<()> {
        let def = match def_ast.value {
            ast::DefinitionValue::Function(func_ast) => Definition::Function(
                self.create_function(def_ast.name.as_str(), func_ast.signature, func_ast.body)?,
            ),
        };
        self.module.defs.insert(def_ast.name.clone(), def);

        Ok(())
    }

    fn create_function(
        &mut self,
        name: &str,
        func_signature: ast::FunctionSignature,
        func_body: Block,
    ) -> CompilationResult<Function<'ctx>> {
        let func_builder = FunctionBuilder::new(self, name, func_signature)?;
        func_builder.attach_body(func_body)?;
        Ok(func_builder.build())
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.module.defs.get(name) {
            Some(def) => Ok(match def {
                Definition::Function(func) => func.clone().into(),
            }),
            None => Err(CompilationError::UnresolvedName(name.to_string())),
        }
    }

    pub fn build(self) -> Module<'ctx> {
        self.module
    }

    #[inline(always)]
    pub fn module_ir(&self) -> &ModuleIR<'ctx> {
        &self.module.module_ir
    }
}

impl<'ctx> Deref for ModuleBuilder<'ctx> {
    type Target = TargetBuilder<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}
