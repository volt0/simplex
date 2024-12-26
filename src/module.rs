use crate::ast;
use crate::compiler::Compiler;
use crate::function::{Function, FunctionCompiler};
use crate::scope::Scope;
use crate::types::Type;
use crate::value::Identifier;
use inkwell::module::Module as ModuleIr;
use inkwell::targets::TargetTriple;
use inkwell::types::{BasicType, BasicTypeEnum};
use std::ops::Deref;
use std::rc::Rc;

pub struct Definition {
    pub name: Rc<str>,
    pub value: DefinitionValue,
}

#[derive(Clone)]
pub enum DefinitionValue {
    Function(Rc<Function>),
}

pub struct Module {
    pub defs: Vec<Definition>,
}

impl Module {
    pub fn compile<'ctx>(&self, compiler: &'ctx Compiler<'ctx>) {
        let module_compiler = ModuleCompiler::compile("foo", self, compiler);
        module_compiler._print_to_stderr();
    }
}

pub struct ModuleCompiler<'ctx> {
    compiler: &'ctx Compiler<'ctx>,
    ir: ModuleIr<'ctx>,
}

impl<'ctx> Deref for ModuleCompiler<'ctx> {
    type Target = Compiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.compiler
    }
}

impl<'ctx> ModuleCompiler<'ctx> {
    fn compile(name: &str, module: &Module, compiler: &'ctx Compiler<'ctx>) -> Self {
        let ir = compiler.create_module(name);
        ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));
        let module_compiler = ModuleCompiler { ir, compiler };

        for def in module.defs.iter() {
            module_compiler.add_definition(def);
        }

        module_compiler
    }
    
    fn add_definition(&self, def: &Definition) {
        let name = def.name.clone();
        let value = def.value.clone();
        match value {
            DefinitionValue::Function(function_ast) => {
            }
        }
        
    }

    pub fn add_function(
        &self,
        name: &str,
        function: Function,
    ) -> FunctionCompiler<'ctx, '_> {
        
        let signature = &function.signature;

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
        let function_compiler = FunctionCompiler::new(function_ir, self);
        
        function_compiler        
    }

    pub fn _print_to_stderr(&self) {
        self.ir.print_to_stderr();
    }
}

impl<'ctx> Scope<'ctx> for ModuleCompiler<'ctx> {
    fn lookup(&self, name: &str) -> Option<Identifier<'ctx>> {
        self.compiler.lookup(name)
    }
}
