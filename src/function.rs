use inkwell::builder::Builder;

use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::function_type::FunctionType;
use crate::types::Type;
use crate::values::{Value, ValueOperations};

type FunctionIR<'ctx> = inkwell::values::FunctionValue<'ctx>;

#[derive(Clone)]
pub struct Function<'ctx> {
    ir: FunctionIR<'ctx>,
    func_type: FunctionType<'ctx>,
}

impl<'ctx> ValueOperations<'ctx> for Function<'ctx> {
    fn binary_operation(
        &self,
        _: &Builder<'ctx>,
        _: BinaryOperation,
        _: &Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>> {
        Err(CompilationError::InvalidOperation)
    }

    fn unary_operation(
        &self,
        _: &Builder<'ctx>,
        _: UnaryOperation,
    ) -> CompilationResult<Value<'ctx>> {
        Err(CompilationError::InvalidOperation)
    }
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
