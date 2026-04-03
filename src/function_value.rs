use crate::function::FunctionSignature;
use crate::value::Value;

pub type FunctionValueIR<'ctx> = inkwell::values::FunctionValue<'ctx>;

#[derive(Clone)]
pub struct FunctionValue<'ctx> {
    pub ir: FunctionValueIR<'ctx>,
    pub signature: FunctionSignature,
}

impl<'ctx> Into<Value<'ctx>> for FunctionValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Function(self)
    }
}

impl<'ctx> FunctionValue<'ctx> {
    pub fn from_ir(
        ir: FunctionValueIR<'ctx>,
        signature: &FunctionSignature,
    ) -> FunctionValue<'ctx> {
        FunctionValue {
            ir,
            signature: signature.clone(),
        }
    }

    pub fn into_ir(self) -> FunctionValueIR<'ctx> {
        self.ir
    }
}
