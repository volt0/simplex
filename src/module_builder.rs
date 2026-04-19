use inkwell::context::Context;
use inkwell::targets::TargetTriple;

use crate::ast;
use crate::definition::Definition;
use crate::errors::{CompilationError, CompilationResult};
use crate::function::Function;
use crate::function_builder::FunctionBuilder;
use crate::function_type::FunctionType;
use crate::module::Module;
use crate::value::Value;

pub struct ModuleBuilder<'ctx> {
    module: Module<'ctx>,
    context: &'ctx Context,
}

impl<'ctx> ModuleBuilder<'ctx> {
    pub fn new(
        context: &'ctx Context,
        module_ast: ast::Module,
    ) -> CompilationResult<ModuleBuilder<'ctx>> {
        let module_ir = context.create_module("test_module");
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        let mut builder = ModuleBuilder {
            context,
            module: Module::new(module_ir),
        };

        for def_ast in module_ast.defs {
            builder.define(def_ast)?;
        }

        Ok(builder)
    }

    #[inline(always)]
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    fn define(&mut self, def_ast: ast::Definition) -> CompilationResult<()> {
        let def = match def_ast.value {
            ast::DefinitionValue::Function(func_ast) => {
                let func_signature = &func_ast.signature;
                let func_type = FunctionType::from_ast(self.context(), func_signature)?;
                let func_type_ir = func_type.ir().clone();
                let func_ir =
                    self.module
                        .module_ir
                        .add_function(def_ast.name.as_str(), func_type_ir, None);

                let func = Function::new(func_ir.clone(), func_type);
                let func_builder = FunctionBuilder::new(func, func_ir, func_signature, self)?;
                func_builder.attach_body(func_ast.body)?;
                Definition::Function(func_builder.build())
            }
        };

        self.module.add_definition(&def_ast.name, def);

        Ok(())
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
