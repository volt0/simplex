use crate::function::{Function, FunctionArgument};
use crate::statement::ValueAssignment;
use std::rc::Rc;

pub trait LocalScope {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem>;
    fn function(self: Rc<Self>) -> Rc<Function>;
}

pub enum LocalScopeItem {
    Argument(Rc<FunctionArgument>),
    Value(Rc<ValueAssignment>),
}
