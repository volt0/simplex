use std::rc::Rc;

use crate::function::Function;

#[derive(Clone)]
pub struct Definition {
    pub name: Rc<str>,
    pub value: DefinitionValue,
}

#[derive(Clone)]
pub enum DefinitionValue {
    Function(Rc<Function>),
}
