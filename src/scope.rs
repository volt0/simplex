use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use crate::value::Identifier;

pub trait Scope<'ctx> {
    fn lookup(&self, name: &str) -> Option<Identifier<'ctx>>;
}

#[derive(Default)]
pub struct LocalScope<'ctx, 'a> {
    items: HashMap<Rc<str>, Identifier<'ctx>>,
    parent: Option<&'a dyn Scope<'ctx>>,
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
