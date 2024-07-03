use std::rc::Rc;

use inkwell::context::Context as BackendContext;
use inkwell::module::Module as ModuleIr;

use crate::function::{Function, FunctionArgument};
use crate::scope::Scope;
use crate::statements::CompoundStatement;
use crate::types::TypeSpec;

#[derive(Clone)]
pub enum DefinitionValue {
    Function(Rc<Function>),
}

#[derive(Clone)]
pub struct Definition {
    name: Rc<str>,
    value: DefinitionValue,
}

impl Definition {
    #[inline(always)]
    pub fn define_function(
        name: Rc<str>,
        args: Vec<Rc<FunctionArgument>>,
        return_type: TypeSpec,
        body: Option<CompoundStatement>,
    ) -> Self {
        Definition {
            name,
            value: DefinitionValue::Function(Function::new(args, return_type, body)),
        }
    }

    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        module_ir: &ModuleIr<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        let name = self.name.as_ref();
        match &self.value {
            DefinitionValue::Function(function) => {
                let function = function.as_ref();
                function.compile(name, scope, &module_ir, ctx);
            }
        }
    }
}
