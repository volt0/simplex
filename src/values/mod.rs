use inkwell::builder::Builder;
use inkwell::types::IntType;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::errors::CompilationError;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn from_ir(ir: BasicValueEnum<'ctx>) -> Self {
        match ir {
            // BasicValueEnum::ArrayValue(_) => {}
            BasicValueEnum::IntValue(ir) => Value::Integer(IntegerValue {
                ir,
                sign_extend: true,
            }),
            // BasicValueEnum::FloatValue(_) => {}
            // BasicValueEnum::PointerValue(_) => {}
            // BasicValueEnum::StructValue(_) => {}
            // BasicValueEnum::VectorValue(_) => {}
            _ => todo!(),
        }
    }

    pub fn as_ir(&self) -> &dyn BasicValue<'ctx> {
        match self {
            Value::Integer(ref inner) => &inner.ir,
        }
    }

    pub fn new_integer(ir: IntValue<'ctx>, sign_extend: bool) -> Self {
        Value::Integer(IntegerValue {
            ir: ir.into(),
            sign_extend,
        })
    }
}

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub ir: IntValue<'ctx>,
    pub sign_extend: bool,
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn compile_upcast(
        &self,
        to_type: IntType<'ctx>,
        sign_extend: bool,
        builder: &Builder<'ctx>,
    ) -> Result<Self, CompilationError> {
        Ok(IntegerValue {
            ir: if sign_extend {
                builder.build_int_s_extend(self.ir, to_type, "")?
            } else {
                builder.build_int_z_extend(self.ir, to_type, "")?
            },
            sign_extend,
        })
    }
}
