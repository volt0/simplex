use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;
use inkwell::types::BasicType;
use inkwell::values::FunctionValue;

use crate::statements::Scope;
use crate::types::Type;

pub struct FunctionArgument {
    pub name: Rc<str>,
    pub arg_type: Type,
}

pub struct Function {
    pub args: Vec<Rc<FunctionArgument>>,
    pub return_type: Type,
    pub body: Scope,
}

impl Function {
    pub fn compile<'ctx>(
        &self,
        name: &str,
        module_ir: &ModuleIr<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> FunctionValue<'ctx> {
        let mut arg_types = vec![];
        for arg in self.args.iter().cloned() {
            arg_types.push(arg.arg_type.compile(ctx).into());
        }

        let is_var_args = false;
        let function_type = match &self.return_type {
            Type::Void => ctx.void_type().fn_type(&arg_types, is_var_args),
            return_type => return_type.compile(ctx).fn_type(&arg_types, is_var_args),
        };

        let function_ir = module_ir.add_function(name, function_type, None);
        let builder = ctx.create_builder();
        self.body.compile(function_ir, &builder, ctx);

        function_ir
    }
}
