use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use inkwell::values::BasicValueEnum;

use crate::ast::FunctionArgument;

pub trait Scope<'ctx> {
    fn lookup(&self, name: &str) -> Option<Identifier<'ctx>>;
}

impl<'ctx, 'a> Scope<'ctx> for LocalScope<'ctx, 'a> {
    fn lookup(&self, name: &str) -> Option<Identifier<'ctx>> {
        if let Some(result) = self.items.get(name).cloned() {
            return Some(result);
        };

        self.parent.and_then(|parent| parent.lookup(name))
    }
}

impl<'ctx, 'a> Deref for LocalScope<'ctx, 'a> {
    type Target = HashMap<Rc<str>, Identifier<'ctx>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'ctx, 'a> DerefMut for LocalScope<'ctx, 'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<'ctx, 'a> LocalScope<'ctx, 'a> {
    pub fn new(parent: &'a dyn Scope<'ctx>) -> Self {
        LocalScope {
            items: Default::default(),
            parent: Some(parent),
        }
    }
}

#[derive(Clone)]
pub enum Identifier<'ctx> {
    Value(Value<'ctx>),
}

impl<'ctx> Identifier<'ctx> {
    pub fn new_argument(arg: FunctionArgument, ir: BasicValueEnum<'ctx>) -> Self {
        Identifier::Value(Value { ir })
    }
}

#[derive(Clone)]
pub struct Value<'ctx> {
    pub ir: BasicValueEnum<'ctx>,
    // pub value_type: Type<'ctx>,
}

#[derive(Default)]
pub struct LocalScope<'ctx, 'a> {
    items: HashMap<Rc<str>, Identifier<'ctx>>,
    parent: Option<&'a dyn Scope<'ctx>>,
}
