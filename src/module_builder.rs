use inkwell::context::Context;
use inkwell::targets::TargetTriple;

use crate::ast;
use crate::basic_block::BasicBlock;
use crate::definition::Definition;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::Function;
use crate::function_builder::FunctionBuilder;
use crate::function_type::FunctionType;
use crate::module::Module;
use crate::value::Value;

pub struct ModuleBuilder<'ctx> {
    context: &'ctx Context,
    module: Module<'ctx>,
}

impl<'ctx> ModuleBuilder<'ctx> {
    fn new(context: &'ctx Context, module: Module<'ctx>) -> Self {
        Self { context, module }
    }

    pub fn from_ast(context: &'ctx Context, module_ast: ast::Module) -> CompilationResult<Self> {
        let module_ir = context.create_module("test_module");
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        let module = Module::new(module_ir);

        let mut module_builder = Self::new(context, module);
        for def_ast in module_ast.defs {
            module_builder.define(def_ast)?;
        }
        Ok(module_builder)
    }

    #[inline(always)]
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    fn define(&mut self, def_ast: ast::Definition) -> CompilationResult<()> {
        let def = match def_ast.value {
            ast::DefinitionValue::Function(func_ast) => Definition::Function(
                self.create_function(def_ast.name.as_str(), func_ast.signature, func_ast.body)?,
            ),
        };
        self.module.add_definition(&def_ast.name, def);

        Ok(())
    }

    fn create_function(
        &mut self,
        name: &str,
        func_signature: ast::FunctionSignature,
        func_body: BasicBlock,
    ) -> CompilationResult<Function<'ctx>> {
        let func_type = FunctionType::from_ast(self.context, &func_signature)?;
        let func_type_ir = func_type.ir().clone();
        let func_ir = self.module.module_ir.add_function(name, func_type_ir, None);
        let func = Function::new(func_ir, func_type);

        let func_builder = FunctionBuilder::new(func, func_signature, self)?;
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
}
