use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use inkwell::values::BasicValueEnum;

use crate::ast::FunctionArgument;

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
pub struct Scope<'ctx, 'a> {
    items: HashMap<Rc<str>, Identifier<'ctx>>,
    parent: Option<&'a Scope<'ctx, 'a>>,
}

impl<'ctx, 'a> Deref for Scope<'ctx, 'a> {
    type Target = HashMap<Rc<str>, Identifier<'ctx>>;

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl<'ctx, 'a> DerefMut for Scope<'ctx, 'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.items
    }
}

impl<'ctx, 'a> Scope<'ctx, 'a> {
    pub fn new(parent: &'a Scope<'ctx, 'a>) -> Self {
        Scope {
            items: Default::default(),
            parent: Some(parent),
        }
    }

    pub fn lookup(&self, name: Rc<str>) -> Option<Identifier<'ctx>> {
        self.items
            .get(&name)
            .cloned()
            .or_else(|| self.parent.unwrap().lookup(name))
    }
}
