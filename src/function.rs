use std::collections::BTreeMap;
use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::types::BasicType;
use inkwell::values::FunctionValue;

use crate::statements::CompoundStatement;
use crate::statements::LocalScope;
use crate::types::Type;
use crate::values::Value;

pub struct FunctionArgument {
    name: Rc<str>,
    arg_type: Type,
}

impl FunctionArgument {
    pub fn new(name: Rc<str>, arg_type: Type) -> Rc<Self> {
        Rc::new(FunctionArgument { name, arg_type })
    }

    pub fn compile<'ctx>(&self, position: u32, function_ir: FunctionValue<'ctx>) -> Value<'ctx> {
        let ir = function_ir.get_nth_param(position).unwrap();
        ir.set_name(self.name.as_ref());
        Value {
            ir: ir.into(),
            value_type: self.arg_type.clone(),
        }
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

        let mut scope = LocalScope {
            index: BTreeMap::new(),
            parent: None,
        };
        for (i, arg) in self.args.iter().enumerate() {
            let value = arg.compile(i as u32, function_ir);
            scope.index.insert(arg.name.clone(), value);
        }

        let builder = ctx.create_builder();
        self.body.compile(&scope, &builder, function_ir, ctx);

        function_ir
    }
}
