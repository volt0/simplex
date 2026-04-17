use inkwell::context::Context;

pub struct Translator {
    context: Context,
}

impl Translator {
    pub fn new() -> Self {
        Self {
            context: Context::create(),
        }
    }

    #[inline(always)]
    pub fn context(&self) -> &Context {
        &self.context
    }
}
