use crate::function::FunctionSignature;

pub type FunctionValueIR<'ctx> = inkwell::values::FunctionValue<'ctx>;

#[derive(Clone)]
pub struct FunctionValue<'ctx> {
    pub ir: FunctionValueIR<'ctx>,
    pub signature: FunctionSignature,
}

impl<'ctx> FunctionValue<'ctx> {
    pub fn into_ir(self) -> FunctionValueIR<'ctx> {
        self.ir
    }
}
