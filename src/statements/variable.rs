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
    pub init: ExpressionRef,
}

impl Variable {
    pub fn new_let(name: Rc<str>, value_type: Option<TypeSpec>, init: ExpressionRef) -> Rc<Self> {
        Rc::new(Variable {
            name,
            value_type,
            init,
        })
    }

    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Value<'ctx> {
        let value = self.init.compile(scope, builder, ctx);
        value.as_ir().set_name(self.name.as_ref());
        value
    }
}
