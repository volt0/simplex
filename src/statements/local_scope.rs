use std::collections::BTreeMap;
use std::rc::Rc;

use crate::scope::Scope;
use crate::values::Value;

pub struct LocalScope<'ctx, 'a> {
    pub index: BTreeMap<Rc<str>, Value<'ctx>>,
    pub parent: &'a dyn Scope<'ctx>,
}

impl<'ctx, 'a> Scope<'ctx> for LocalScope<'ctx, 'a> {
    fn resolve(&self, name: Rc<str>) -> &Value<'ctx> {
        if let Some(value) = self.index.get(name.as_ref()) {
            return value;
        }

        self.parent.resolve(name)
    }
}
