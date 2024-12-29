use crate::function::FunctionArgument;
use crate::statement::ValueAssignment;
use std::rc::Rc;

pub trait LocalScope {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem>;
}

pub enum LocalScopeItem {
    Argument(Rc<FunctionArgument>),
    Value(Rc<ValueAssignment>),
}
