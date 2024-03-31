use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::BasicValueEnum;

use crate::errors::CompilationError;
use crate::expressions::{ExpressionRef, Value};
use crate::scope::Scope;

#[allow(unused)]
#[derive(Debug)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
    ShiftLeft,
    ShiftRight,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    LogicalAnd,
    LogicalOr,
}

pub struct BinaryOperationExpression {
    pub(super) operation: BinaryOperation,
    pub(super) a: ExpressionRef,
    pub(super) b: ExpressionRef,
}

impl BinaryOperationExpression {
    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Result<Value<'ctx>, CompilationError> {
        let a = self.a.compile(scope, builder, ctx);
        let b = self.b.compile(scope, builder, ctx);
        assert_eq!(a.get_type(), b.get_type());

        match a.ir {
            // BasicValueEnum::ArrayValue(_) => {}
            BasicValueEnum::IntValue(_) => {
                let a_ir = a.ir.into_int_value();
                let b_ir = b.ir.into_int_value();

                let result = match self.operation {
                    BinaryOperation::Add => builder.build_int_add(a_ir, b_ir, "")?,
                    BinaryOperation::Sub => builder.build_int_sub(a_ir, b_ir, "")?,
                    BinaryOperation::Mul => builder.build_int_mul(a_ir, b_ir, "")?,
                    // BinaryOperation::Div => builder.build_int_unsigned_div(a_ir, b_ir, "")?,
                    // BinaryOperation::Mod => builder.build_int_unsigned_rem(a_ir, b_ir, "")?,
                    // BinaryOperation::BitAnd => {}
                    // BinaryOperation::BitXor => {}
                    // BinaryOperation::BitOr => {}
                    // BinaryOperation::ShiftLeft => {}
                    // BinaryOperation::ShiftRight => {}
                    // BinaryOperation::Eq => {}
                    // BinaryOperation::Ne => {}
                    // BinaryOperation::Gt => {}
                    // BinaryOperation::Ge => {}
                    // BinaryOperation::Lt => {}
                    // BinaryOperation::Le => {}
                    // BinaryOperation::LogicalAnd => {}
                    // BinaryOperation::LogicalOr => {}
                    _ => todo!(),
                };

                Ok(Value::from_ir(result.into()))
            }
            // BasicValueEnum::FloatValue(_) => {}
            // BasicValueEnum::FunctionValue(_) => {}
            // BasicValueEnum::PointerValue(_) => {}
            // BasicValueEnum::StructValue(_) => {}
            // BasicValueEnum::VectorValue(_) => {}
            _ => todo!(),
        }
    }
}
