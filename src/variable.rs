use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

use crate::expressions::ExpressionRef;
use crate::scope::Scope;
use crate::types::TypeSpec;
use crate::values::Value;

pub struct Variable {
    pub name: Rc<str>,
    pub value_type: Option<TypeSpec>,
    pub value_init: ExpressionRef,
}

impl Variable {
    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Value<'ctx> {
        let value = self.value_init.compile(scope, builder, ctx);
        value.set_name(self.name.as_ref());
        value
    }
}
