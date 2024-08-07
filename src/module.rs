use std::ops::Deref;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::targets::TargetTriple;

use crate::ast;
use crate::function::Function;
use crate::scope::Scope;

pub struct Module<'ctx> {
    ir: ModuleIr<'ctx>,
}

impl<'ctx> Deref for Module<'ctx> {
    type Target = ModuleIr<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.ir
    }
}

impl<'ctx, 's> Module<'ctx> {
    pub fn compile(
        name: &str,
        module_ast: ast::Module,
        scope: &Scope<'ctx, 's>,
        ctx: &'ctx BackendContext,
    ) -> Self {
        let ir = ctx.create_module(name);
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));
        let module = Module { ir };

        for definition in module_ast.defs {
            let name = definition.name;
            match definition.value {
                ast::DefinitionImpl::Function(function_ast) => {
                    let signature = function_ast.signature;
                    let payload = function_ast.payload;

                    let function = Function::new(name.as_ref(), signature, &module, ctx);
                    if let Some(payload) = payload {
                        function.compile(payload, scope, &module, ctx);
                    }
                }
            }
        }

        module
    }
}
