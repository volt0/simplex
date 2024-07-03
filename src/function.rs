use std::collections::BTreeMap;
use std::rc::Rc;

use inkwell::context::{Context as BackendContext, Context};
use inkwell::module::Module as ModuleIr;
use inkwell::types::{BasicType, FunctionType};
use inkwell::values::FunctionValue;

use crate::scope::Scope;
use crate::statements::CompoundStatement;
use crate::types::TypeSpec;
use crate::values::Value;

pub struct FunctionArgument {
    name: Rc<str>,
    arg_type: TypeSpec,
}

impl FunctionArgument {
    pub fn new(name: Rc<str>, arg_type: TypeSpec) -> Rc<Self> {
        Rc::new(FunctionArgument { name, arg_type })
    }

    pub fn compile<'ctx>(&self, position: u32, function_ir: FunctionValue<'ctx>) -> Value<'ctx> {
        let ir = function_ir.get_nth_param(position).unwrap();
        ir.set_name(self.name.as_ref());

        // fixme
        Value::from_ir(ir)
    }
}

pub struct FunctionScope<'ctx, 'a> {
    pub index: BTreeMap<Rc<str>, Value<'ctx>>,
    pub parent: &'a dyn Scope<'ctx>,
}

impl<'ctx, 'a> Scope<'ctx> for FunctionScope<'ctx, 'a> {
    fn resolve(&self, name: Rc<str>) -> &Value<'ctx> {
        if let Some(value) = self.index.get(name.as_ref()) {
            return value;
        }

        self.parent.resolve(name)
    }
}

pub struct Function {
    args: Vec<Rc<FunctionArgument>>,
    return_type: TypeSpec,
    body: Option<CompoundStatement>,
}

impl Function {
    pub fn new(
        args: Vec<Rc<FunctionArgument>>,
        return_type: TypeSpec,
        body: Option<CompoundStatement>,
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
        scope: &dyn Scope<'ctx>,
        module_ir: &ModuleIr<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> FunctionValue<'ctx> {
        let type_ir = self.compile_type(ctx);
        let function_ir = module_ir.add_function(name, type_ir, None);

        let mut scope = FunctionScope {
            index: BTreeMap::new(),
            parent: scope,
        };
        for (i, arg) in self.args.iter().enumerate() {
            let value = arg.compile(i as u32, function_ir);
            scope.index.insert(arg.name.clone(), value);
        }

        let builder = ctx.create_builder();
        if let Some(body) = &self.body {
            body.compile(&scope, &builder, function_ir, ctx);
        }

        function_ir
    }

    fn compile_type<'ctx>(&self, ctx: &'ctx Context) -> FunctionType<'ctx> {
        let mut arg_types = vec![];
        for arg in self.args.iter() {
            arg_types.push(arg.arg_type.compile(ctx).into());
        }

        let is_var_args = false;
        match &self.return_type {
            TypeSpec::Void => ctx.void_type().fn_type(&arg_types, is_var_args),
            return_type => return_type.compile(ctx).fn_type(&arg_types, is_var_args),
        }
    }
}
