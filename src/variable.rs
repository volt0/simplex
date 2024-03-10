use std::cell::OnceCell;
use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::BasicValueEnum;

use crate::cache::{Cache, ValueCacheKey};
use crate::expressions::Expression;
use crate::types::Type;

pub struct Variable {
    pub name: Rc<str>,
    pub value_type: Type,
    pub value: Box<Expression>,
    pub cache_key: OnceCell<ValueCacheKey>,
}

impl Variable {
    pub fn compile<'ctx>(
        &self,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
        cache: &mut Cache<'ctx>,
    ) {
        let ir = self.value.compile(builder, ctx, cache);
        let ir: BasicValueEnum = ir.try_into().unwrap();
        ir.set_name(self.name.as_ref());

        let cache_key = cache.values.insert(ir.into());
        self.cache_key.set(cache_key).unwrap()
    }

    pub fn cache_key(&self) -> ValueCacheKey {
        self.cache_key.get().unwrap().clone()
    }
}
