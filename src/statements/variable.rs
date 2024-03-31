use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

use crate::expressions::ExpressionRef;
use crate::expressions::Value;
use crate::scope::Scope;
use crate::types::TypeSpec;

pub struct Variable {
    pub name: Rc<str>,
    pub value_type: Option<TypeSpec>,
    pub value_init: ExpressionRef,
}

impl Variable {
    pub fn new(name: Rc<str>, value_type: Option<TypeSpec>, value_init: ExpressionRef) -> Rc<Self> {
        Rc::new(Variable {
            name,
            value_type,
            value_init,
        })
    }

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
