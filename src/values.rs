#![allow(unused)]

use std::cell::OnceCell;
use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::AnyValueEnum;

use crate::cache::Cache;
use crate::function::FunctionArgument;
use crate::variable::Variable;

pub struct Identifier {
    pub name: Rc<str>,
    pub value: OnceCell<Value>,
}

impl Identifier {
    pub fn compile<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
        cache: &Cache<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        self.value.get().unwrap().compile(builder, ctx, cache)
    }
}

pub enum Value {
    Argument(Rc<FunctionArgument>),
    Variable(Rc<Variable>),
}

impl Value {
    pub fn compile<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
        cache: &Cache<'ctx>,
    ) -> AnyValueEnum<'ctx> {
        let cache_key = match self {
            Value::Argument(arg) => arg.cache_key(),
            Value::Variable(var) => var.cache_key(),
        };
        cache.values.get(cache_key).unwrap().clone()
    }
}
