use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::IntType;
use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

use crate::expression::BinaryOperation;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntegerValue<'ctx>),
}

impl<'ctx> Value<'ctx> {
    pub fn into_ir(self) -> BasicValueEnum<'ctx> {
        match self {
            Value::Integer(value) => value.into(),
        }
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Self {
        match self {
            Value::Integer(value) => match other {
                Value::Integer(other) => value
                    .binary_operation(operation, other, builder, context)
                    .into(),
            },
        }
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone)]
pub struct IntegerType {
    pub is_signed: bool,
    pub width: IntegerTypeSize,
}

impl IntegerType {
    pub fn to_ir<'ctx>(&self, context: &'ctx Context) -> IntType<'ctx> {
        match self.width {
            IntegerTypeSize::I8 => context.i8_type(),
            IntegerTypeSize::I16 => context.i16_type(),
            IntegerTypeSize::I32 => context.i32_type(),
            IntegerTypeSize::I64 => context.i64_type(),
        }
    }
}

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub(crate) ir: IntValue<'ctx>,
    pub value_type: IntegerType,
}

impl<'ctx> Into<Value<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> Value<'ctx> {
        Value::Integer(self)
    }
}

impl<'ctx> Into<BasicValueEnum<'ctx>> for IntegerValue<'ctx> {
    fn into(self) -> BasicValueEnum<'ctx> {
        self.ir.as_basic_value_enum()
    }
}

impl<'ctx> IntegerValue<'ctx> {
    pub fn from_constant(value: i32, context: &'ctx Context) -> Self {
        IntegerValue {
            ir: context.i32_type().const_int(value as u64, true),
            value_type: IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I32,
            },
        }
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &IntegerValue<'ctx>,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> Self {
        let lhs_type = self.value_type.clone();
        let rhs_type = other.value_type.clone();
        let result_type = if lhs_type.is_signed == rhs_type.is_signed {
            if rhs_type.width > lhs_type.width {
                rhs_type
            } else {
                lhs_type
            }
        } else if rhs_type.is_signed && rhs_type.width > lhs_type.width {
            rhs_type
        } else if lhs_type.is_signed && lhs_type.width > rhs_type.width {
            lhs_type
        } else {
            unimplemented!();
        };

        let lhs_ir = self.to_ir_expanded(&result_type, builder, context);
        let rhs_ir = other.to_ir_expanded(&result_type, builder, context);
        let result_ir = match operation {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, "").unwrap(),
        };

        IntegerValue {
            ir: result_ir,
            value_type: result_type,
        }
    }

    fn to_ir_expanded(
        &self,
        new_type: &IntegerType,
        builder: &Builder<'ctx>,
        context: &'ctx Context,
    ) -> IntValue<'ctx> {
        if new_type.is_signed {
            builder
                .build_int_s_extend(self.ir, new_type.to_ir(context), "")
                .unwrap()
        } else {
            builder
                .build_int_z_extend(self.ir, new_type.to_ir(context), "")
                .unwrap()
        }
    }
}
