use std::cell::OnceCell;
use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::types::BasicType;
use inkwell::values::FunctionValue;

use crate::cache::{Cache, ValueCacheKey};
use crate::statements::CompoundStatement;
use crate::types::Type;

pub struct FunctionArgument {
    name: Rc<str>,
    arg_type: Type,
    cache_key: OnceCell<ValueCacheKey>,
}

impl FunctionArgument {
    pub fn new(name: Rc<str>, arg_type: Type) -> Rc<Self> {
        Rc::new(FunctionArgument {
            name,
            arg_type,
            cache_key: OnceCell::new(),
        })
    }

    pub fn compile<'ctx>(
        &self,
        position: u32,
        function_ir: FunctionValue<'ctx>,
        cache: &mut Cache<'ctx>,
    ) {
        let arg_ir = function_ir.get_nth_param(position).unwrap();
        arg_ir.set_name(self.name.as_ref());

        let arg_cache_key = cache.values.insert(arg_ir.into());
        self.cache_key.set(arg_cache_key).unwrap()
    }

    pub fn cache_key(&self) -> ValueCacheKey {
        self.cache_key.get().unwrap().clone()
    }
}

pub struct Function {
    args: Vec<Rc<FunctionArgument>>,
    return_type: Type,
    body: CompoundStatement,
}

impl Function {
    pub fn new(
        args: Vec<Rc<FunctionArgument>>,
        return_type: Type,
        body: CompoundStatement,
    ) -> Rc<Self> {
        Rc::new(Function {
            args,
            return_type,
            body,
        })
    }

    pub fn compile<'ctx>(
        &self,
        name: &str,
        module_ir: &ModuleIr<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> FunctionValue<'ctx> {
        let mut arg_types = vec![];
        for arg in self.args.iter() {
            arg_types.push(arg.arg_type.compile(ctx).into());
        }

        let is_var_args = false;
        let function_type = match &self.return_type {
            Type::Void => ctx.void_type().fn_type(&arg_types, is_var_args),
            return_type => return_type.compile(ctx).fn_type(&arg_types, is_var_args),
        };

        let function_ir = module_ir.add_function(name, function_type, None);

        let mut cache = Cache::default();
        for (i, arg) in self.args.iter().enumerate() {
            arg.compile(i as u32, function_ir, &mut cache);
        }

        let builder = ctx.create_builder();
        self.body.compile(function_ir, &builder, ctx, &mut cache);

        function_ir
    }
}
