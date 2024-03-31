use std::rc::Rc;

use crate::expressions::Value;

pub trait Scope<'ctx> {
    fn resolve(&self, name: Rc<str>) -> &Value<'ctx>;
}
