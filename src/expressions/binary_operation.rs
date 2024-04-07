use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;

use crate::errors::CompilationError;
use crate::expressions::{ExpressionRef, Value};
use crate::scope::Scope;
use crate::values::IntegerValue;

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

impl<'ctx> BinaryOperationExpr {
    pub fn compile(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Result<Value<'ctx>, CompilationError> {
        let a = self.a.compile(scope, builder, ctx);
        let b = self.b.compile(scope, builder, ctx);
        match a {
            Value::Integer(a) => self.compile_integer(a, b, builder),
        }
    }

    fn compile_integer(
        &self,
        a: IntegerValue<'ctx>,
        b: Value<'ctx>,
        builder: &Builder<'ctx>,
    ) -> Result<Value<'ctx>, CompilationError> {
        let (a, b) = match b {
            Value::Integer(b) => {
                if a.sign_extend && !b.sign_extend {
                    return Err(CompilationError::TypeMismatch());
                }

                let a_type = a.ir.get_type();
                let b_type = b.ir.get_type();

                if a_type == b_type {
                    (a, b)
                } else {
                    if a_type.get_bit_width() > b_type.get_bit_width() {
                        (a.clone(), b.compile_upcast(a_type, b.sign_extend, builder)?)
                    } else {
                        (a.compile_upcast(b_type, b.sign_extend, builder)?, b.clone())
                    }
                }
            }
        };

        let result = {
            match self.operation {
                BinaryOperation::Add => builder.build_int_add(a.ir, b.ir, "")?,
                BinaryOperation::Sub => builder.build_int_sub(a.ir, b.ir, "")?,
                BinaryOperation::Mul => builder.build_int_mul(a.ir, b.ir, "")?,
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
            }
        };
        Ok(Value::new_integer(result, a.sign_extend))
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
