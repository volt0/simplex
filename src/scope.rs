use crate::function::{Function, FunctionArgument};
use crate::statement::ValueAssignment;
use std::rc::Rc;

pub trait LocalScope {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem>;
    fn function(&self) -> Rc<Function>;
}

#[derive(Clone)]
pub enum LocalScopeItem {
    Argument(Rc<FunctionArgument>),
    Value(Rc<ValueAssignment>),
}

impl From<&Rc<ValueAssignment>> for LocalScopeItem {
    fn from(value: &Rc<ValueAssignment>) -> Self {
        LocalScopeItem::Value(value.clone())
    }
}
