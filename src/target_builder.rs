use std::collections::HashMap;

use inkwell::context::Context;
use inkwell::targets::TargetTriple;

use crate::ast;
use crate::errors::{CompilationError, CompilationResult};
use crate::module::Module;
use crate::module_builder::ModuleBuilder;
use crate::types::Type;

pub struct TargetBuilder<'ctx> {
    context: &'ctx Context,
    builtin_types: HashMap<String, Type<'ctx>>,
}

impl<'ctx> TargetBuilder<'ctx> {
    pub fn new(context: &'ctx Context) -> TargetBuilder<'ctx> {
        let builtin_types = HashMap::from_iter([
            ("i8".to_string(), Type::new_i8(context, true)),
            ("i16".to_string(), Type::new_i16(context, true)),
            ("i32".to_string(), Type::new_i32(context, true)),
            ("i64".to_string(), Type::new_i64(context, true)),
            ("u8".to_string(), Type::new_i8(context, false)),
            ("u16".to_string(), Type::new_i16(context, false)),
            ("u32".to_string(), Type::new_i32(context, false)),
            ("u64".to_string(), Type::new_i64(context, false)),
            ("f32".to_string(), Type::new_f32(context)),
            ("f64".to_string(), Type::new_f64(context)),
            ("bool".to_string(), Type::new_bool(context)),
        ]);

        TargetBuilder {
            context,
            builtin_types,
        }
    }

    #[inline(always)]
    pub fn context(&self) -> &'ctx Context {
        self.context
    }

    pub fn create_module(
        &self,
        name: &str,
        module_ast: ast::Module,
    ) -> CompilationResult<Module<'_>> {
        let module_ir = self.context.create_module(name);
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        let module = Module::new(module_ir);

        let mut module_builder = ModuleBuilder::new(self, module);
        for def_ast in module_ast.defs {
            module_builder.define(def_ast)?;
        }

        Ok(module_builder.build())
    }

    pub fn load_type(&self, name: &str) -> CompilationResult<Type<'ctx>> {
        match self.builtin_types.get(name) {
            Some(value_type) => Ok(value_type.clone()),
            None => Err(CompilationError::UnresolvedName(name.to_string())),
        }
    }
}
