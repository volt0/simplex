use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

use crate::expressions::Expression;
use crate::scope::Scope;
use crate::types::Type;
use crate::values::Value;

pub struct Variable {
    pub name: Rc<str>,
    pub value_type: Type,
    pub value_init: Box<Expression>,
}

impl Variable {
    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Value<'ctx> {
        let value = self.value_init.compile(scope, builder, ctx);
        let ir = value.compile_as_basic();
        ir.set_name(self.name.as_ref());
        value
    }
}
