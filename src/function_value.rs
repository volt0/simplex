use inkwell::values::AnyValueEnum;

use crate::function::FunctionSignature;
use crate::value::Value;

pub type FunctionValueIR<'ctx> = inkwell::values::FunctionValue<'ctx>;

#[derive(Clone)]
pub struct FunctionValue<'ctx> {
    pub ir: FunctionValueIR<'ctx>,
    pub signature: FunctionSignature,
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
    pub fn new(ir: FunctionValueIR<'ctx>, signature: &FunctionSignature) -> FunctionValue<'ctx> {
        FunctionValue {
            ir,
            signature: signature.clone(),
        }
    }
}
