use crate::ast;
use crate::compiler::Compiler;
use crate::function::Function;
use crate::types::Type;
use inkwell::module::Module as ModuleIr;
use inkwell::targets::TargetTriple;
use inkwell::types::{BasicType, BasicTypeEnum};
use std::ops::Deref;

pub struct Module<'ctx> {
    compiler: &'ctx Compiler<'ctx>,
    ir: ModuleIr<'ctx>,
}

impl<'ctx> Deref for Module<'ctx> {
    type Target = Compiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.compiler
    }
}

impl<'ctx> Module<'ctx> {
    pub fn compile(name: &str, module_ast: ast::Module, compiler: &'ctx Compiler<'ctx>) -> Self {
        let ir = compiler.create_module(name);
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));
        let module = Module { ir, compiler };

        for definition in module_ast.defs {
            let name = definition.name;
            match definition.value {
                ast::DefinitionImpl::Function(function_ast) => {
                    let signature = function_ast.signature;
                    let payload = function_ast.payload;

                    let function = module.add_function(name.as_ref(), signature);
                    if let Some(payload) = payload {
                        function.compile(payload, module.compiler, &module, module.compiler);
                    }
                }
            }
        }

        module
    }

    pub fn add_function(&self, name: &str, signature: ast::FunctionSignature) -> Function<'ctx> {
        let type_ir = {
            let ctx = self.context();
            let signature = signature.clone();

            let mut arg_types = vec![];
            for arg_ast in signature.args {
                let arg_type = Type::compile(arg_ast.type_spec, &ctx);
                let arg_type_ir: BasicTypeEnum = arg_type.try_into().unwrap();
                arg_types.push(arg_type_ir.into());
            }

            let return_type_ast = signature.return_type.unwrap_or(ast::TypeSpec::Void);
            let return_type = Type::compile(return_type_ast, &ctx);

            let is_var_args = false;
            match return_type {
                Type::Void(_) => ctx.void_type().fn_type(&arg_types, is_var_args),
                return_type => {
                    let return_type_ir: BasicTypeEnum = return_type.try_into().unwrap();
                    return_type_ir.fn_type(&arg_types, is_var_args)
                }
            }
        };

        let function_ir = self.ir.add_function(name, type_ir, None);
        Function::new(function_ir, signature)
    }

    pub fn _print_to_stderr(&self) {
        self.ir.print_to_stderr();
    }
}
