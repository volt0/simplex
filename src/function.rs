use std::collections::HashMap;

use crate::function_type::FunctionType;
use crate::value::Value;

pub type FunctionIR<'ctx> = inkwell::values::FunctionValue<'ctx>;

#[derive(Clone)]
pub struct Function<'ctx> {
    ir: FunctionIR<'ctx>,
    func_type: FunctionType<'ctx>,
    pub args: HashMap<String, Value<'ctx>>,
}

impl<'ctx> Function<'ctx> {
    pub fn new(ir: FunctionIR<'ctx>, func_type: FunctionType<'ctx>) -> Self {
        Self {
            ir,
            func_type,
            args: HashMap::new(),
        }
    }

    pub fn get_type(&self) -> &FunctionType<'ctx> {
        &self.func_type
    }

    #[inline(always)]
    pub fn ir(&self) -> &FunctionIR<'ctx> {
        &self.ir
    }
}

impl<'ctx> Into<FunctionIR<'ctx>> for Function<'ctx> {
    fn into(self) -> FunctionIR<'ctx> {
        self.ir
    }
}

impl<'ctx> Into<Value<'ctx>> for Function<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Function(self)
    }
}
