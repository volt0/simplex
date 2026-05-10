use inkwell::builder::Builder;

use crate::errors::CompilationResult;
use crate::expression::{BinaryOperation, UnaryOperation};

use super::Value;

pub trait ValueOperations<'ctx> {
    fn binary_operation(
        &self,
        builder: &Builder<'ctx>,
        op: BinaryOperation,
        other: &Value<'ctx>,
    ) -> CompilationResult<Value<'ctx>>;

    fn unary_operation(
        &self,
        builder: &Builder<'ctx>,
        op: UnaryOperation,
    ) -> CompilationResult<Value<'ctx>>;
}
