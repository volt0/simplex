use inkwell::values::AnyValueEnum;
use slotmap::{DefaultKey, SlotMap};

pub type ValueCacheKey = DefaultKey;

#[derive(Default)]
pub struct Cache<'ctx> {
    pub values: SlotMap<DefaultKey, AnyValueEnum<'ctx>>,
}
