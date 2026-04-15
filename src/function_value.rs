use inkwell::values::AnyValueEnum;

use crate::function_type::FunctionType;
use crate::value::Value;

pub type FunctionValueIR<'ctx> = inkwell::values::FunctionValue<'ctx>;

#[derive(Clone)]
pub struct FunctionValue<'ctx> {
    ir: FunctionValueIR<'ctx>,
    func_type: FunctionType<'ctx>,
}

impl<'ctx> Into<AnyValueEnum<'ctx>> for FunctionValue<'ctx> {
    fn into(self) -> AnyValueEnum<'ctx> {
        AnyValueEnum::FunctionValue(self.ir)
    }
}

impl<'ctx> Into<FunctionValueIR<'ctx>> for FunctionValue<'ctx> {
    fn into(self) -> FunctionValueIR<'ctx> {
        self.ir
    }
}

impl<'ctx> Into<Value<'ctx>> for FunctionValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Function(self)
    }
}

impl<'ctx> FunctionValue<'ctx> {
    pub fn new(ir: FunctionValueIR<'ctx>, func_type: FunctionType<'ctx>) -> FunctionValue<'ctx> {
        FunctionValue { ir, func_type }
    }

    pub fn get_type(&self) -> &FunctionType<'ctx> {
        &self.func_type
    }
}
