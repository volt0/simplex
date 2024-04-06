use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

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

pub struct BinaryOperationExpr {
    operation: BinaryOperation,
    a: ExpressionRef,
    b: ExpressionRef,
}

impl BinaryOperationExpr {
    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Result<Value<'ctx>, CompilationError> {
        let a = self.a.compile(scope, builder, ctx);
        let b = self.b.compile(scope, builder, ctx);
        // assert_eq!(a.get_type(), b.get_type());
        todo!()

        // match a.ir {
        //     // BasicValueEnum::ArrayValue(_) => {}
        //     BasicValueEnum::IntValue(_) => {
        //         let a_ir = a.ir.into_int_value();
        //         let b_ir = b.ir.into_int_value();
        //
        //         let result = match self.operation {
        //             BinaryOperation::Add => builder.build_int_add(a_ir, b_ir, "")?,
        //             BinaryOperation::Sub => builder.build_int_sub(a_ir, b_ir, "")?,
        //             BinaryOperation::Mul => builder.build_int_mul(a_ir, b_ir, "")?,
        //             // BinaryOperation::Div => builder.build_int_unsigned_div(a_ir, b_ir, "")?,
        //             // BinaryOperation::Mod => builder.build_int_unsigned_rem(a_ir, b_ir, "")?,
        //             // BinaryOperation::BitAnd => {}
        //             // BinaryOperation::BitXor => {}
        //             // BinaryOperation::BitOr => {}
        //             // BinaryOperation::ShiftLeft => {}
        //             // BinaryOperation::ShiftRight => {}
        //             // BinaryOperation::Eq => {}
        //             // BinaryOperation::Ne => {}
        //             // BinaryOperation::Gt => {}
        //             // BinaryOperation::Ge => {}
        //             // BinaryOperation::Lt => {}
        //             // BinaryOperation::Le => {}
        //             // BinaryOperation::LogicalAnd => {}
        //             // BinaryOperation::LogicalOr => {}
        //             _ => todo!(),
        //         };
        //
        //         Ok(Value::from_ir(result.into()))
        //     }
        //     // BasicValueEnum::FloatValue(_) => {}
        //     // BasicValueEnum::FunctionValue(_) => {}
        //     // BasicValueEnum::PointerValue(_) => {}
        //     // BasicValueEnum::StructValue(_) => {}
        //     // BasicValueEnum::VectorValue(_) => {}
        //     _ => todo!(),
        // }
    }
}

impl BinaryOperationExpr {
    #[inline(always)]
    pub fn new_add(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Add,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_sub(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Sub,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_mul(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Mul,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_div(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Div,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_mod(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Mod,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_bit_and(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::BitAnd,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_bit_xor(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::BitXor,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_shift_left(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::ShiftLeft,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_shift_right(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::ShiftRight,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_bit_or(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::BitOr,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_lt(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Lt,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_gt(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Gt,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_le(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Le,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_ge(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Ge,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_eq(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Eq,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_ne(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::Ne,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_logic_and(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::LogicalAnd,
            a,
            b,
        }
    }

    #[inline(always)]
    pub fn new_logic_or(a: ExpressionRef, b: ExpressionRef) -> Self {
        BinaryOperationExpr {
            operation: BinaryOperation::LogicalOr,
            a,
            b,
        }
    }
}
