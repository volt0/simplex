use std::rc::Rc;

use crate::values::Value;

pub trait Scope<'ctx> {
    fn resolve(&self, name: Rc<str>) -> &Value<'ctx>;
}
