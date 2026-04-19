use crate::errors::CompilationResult;
use crate::function_value::FunctionValue;

pub struct Function<'ctx> {
    pub inner: FunctionValue<'ctx>,
}

impl<'ctx> Function<'ctx> {
    pub fn from_ast(inner: FunctionValue<'ctx>) -> CompilationResult<Self> {
        Ok(Self::new(inner))
    }

    pub fn new(inner: FunctionValue<'ctx>) -> Self {
        Self { inner }
    }
}
