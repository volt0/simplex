use crate::function_type::FunctionType;
use crate::types::Type;
use crate::value::Value;

type FunctionIR<'ctx> = inkwell::values::FunctionValue<'ctx>;

#[derive(Clone)]
pub struct Function<'ctx> {
    ir: FunctionIR<'ctx>,
    func_type: FunctionType<'ctx>,
}

impl<'ctx> Function<'ctx> {
    pub fn new(ir: FunctionIR<'ctx>, func_type: FunctionType<'ctx>) -> Self {
        Self { ir, func_type }
    }

    pub fn get_type(&self) -> &FunctionType<'ctx> {
        &self.func_type
    }

    pub fn get_return_type(&self) -> &Type<'ctx> {
        self.func_type.return_type()
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
