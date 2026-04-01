use inkwell::values::{BasicValue, BasicValueEnum, IntValue};
use inkwell::IntPredicate;

use crate::bool_value::BoolValue;
use crate::errors::{CompilationError, CompilationResult};
use crate::expression::{BinaryOperation, UnaryOperation};
use crate::expression_translator::ExpressionTranslator;
use crate::float_type::FloatType;
use crate::float_value::FloatValue;
use crate::integer_type::{IntegerType, IntegerTypeSize};
use crate::types::Type;
use crate::value::Value;

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub ir: IntValue<'ctx>,
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
    pub fn from_ir(
        value_ir: BasicValueEnum<'ctx>,
        value_type: &IntegerType,
    ) -> CompilationResult<Self> {
        if let BasicValueEnum::IntValue(value_ir) = value_ir {
            return Ok(IntegerValue {
                ir: value_ir,
                value_type: value_type.clone(),
            });
        }
        panic!("Expected IntValue, got {:?}", value_ir);
    }

    pub fn from_constant(
        value: i32,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> Self {
        let context = expression_translator.context();
        IntegerValue {
            ir: context.i32_type().const_int(value as u64, true),
            value_type: IntegerType {
                is_signed: true,
                width: IntegerTypeSize::I32,
            },
        }
    }

    pub fn to_bool(
        &self,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<BoolValue<'ctx>> {
        let builder = expression_translator.builder();
        let context = expression_translator.context();
        Ok(BoolValue {
            ir: builder.build_int_compare(
                IntPredicate::NE,
                self.ir,
                self.value_type.to_ir(context).const_int(0, false),
                "",
            )?,
        })
    }

    pub fn to_float(
        &self,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<FloatValue<'ctx>> {
        let result_type = match self.value_type.width {
            IntegerTypeSize::I8 | IntegerTypeSize::I16 => FloatType::F32,
            IntegerTypeSize::I32 => FloatType::F64,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let builder = expression_translator.builder();
        let context = expression_translator.context();
        let result_ir = if self.value_type.is_signed {
            builder.build_signed_int_to_float(self.ir, result_type.to_ir(context), "")?
        } else {
            builder.build_unsigned_int_to_float(self.ir, result_type.to_ir(context), "")?
        };

        Ok(FloatValue {
            ir: result_ir,
            value_type: result_type,
        }
        .into())
    }

    pub fn binary_operation(
        &self,
        operation: BinaryOperation,
        other: &Value<'ctx>,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let other = match other {
            Value::Integer(other) => other.clone(),
            Value::Bool(other) => other.to_integer(expression_translator, expression_type)?,
            _ => return Err(CompilationError::TypeMismatch),
        };

        let lhs_type = self.value_type.clone();
        let rhs_type = other.value_type.clone();
        let result_type = match expression_type {
            None => {
                if lhs_type.is_signed == rhs_type.is_signed {
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
                    return Err(CompilationError::TypeMismatch);
                }
            }

            Some(expression_type) => match expression_type {
                Type::Integer(expression_type) => expression_type.clone(),
                _ => return Err(CompilationError::TypeMismatch),
            },
        };

        let builder = expression_translator.builder();
        let lhs_ir = self.to_ir_expanded(&result_type, expression_translator)?;
        let rhs_ir = other.to_ir_expanded(&result_type, expression_translator)?;
        let result_ir = match operation {
            BinaryOperation::Add => builder.build_int_add(lhs_ir, rhs_ir, ""),
            BinaryOperation::Sub => builder.build_int_sub(lhs_ir, rhs_ir, ""),
            BinaryOperation::Mul => builder.build_int_mul(lhs_ir, rhs_ir, ""),
            BinaryOperation::Div => {
                if self.value_type.is_signed {
                    builder.build_int_signed_div(lhs_ir, rhs_ir, "")
                } else {
                    builder.build_int_unsigned_div(lhs_ir, rhs_ir, "")
                }
            }
            BinaryOperation::Mod => {
                if self.value_type.is_signed {
                    builder.build_int_signed_rem(lhs_ir, rhs_ir, "")
                } else {
                    builder.build_int_unsigned_rem(lhs_ir, rhs_ir, "")
                }
            }
            BinaryOperation::BitAnd => builder.build_and(lhs_ir, rhs_ir, ""),
            BinaryOperation::BitXor => builder.build_xor(lhs_ir, rhs_ir, ""),
            BinaryOperation::BitOr => builder.build_or(lhs_ir, rhs_ir, ""),
            BinaryOperation::ShiftLeft => builder.build_left_shift(lhs_ir, rhs_ir, ""),
            BinaryOperation::ShiftRight => {
                builder.build_right_shift(lhs_ir, rhs_ir, self.value_type.is_signed, "")
            }
        };

        Ok(IntegerValue {
            ir: result_ir?,
            value_type: result_type,
        }
        .into())
    }

    pub fn unary_operation(
        &self,
        operation: UnaryOperation,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
        expression_type: Option<&Type>,
    ) -> CompilationResult<Value<'ctx>> {
        let arg_type = match expression_type {
            None => self.value_type.clone(),
            Some(expression_type) => match expression_type {
                Type::Integer(expression_type) => expression_type.clone(),
                _ => return Err(CompilationError::TypeMismatch),
            },
        };

        let builder = expression_translator.builder();
        let arg_ir = self.to_ir_expanded(&arg_type, expression_translator)?;
        let result_ir = match operation {
            UnaryOperation::Plus => arg_ir,
            UnaryOperation::Minus => builder.build_int_neg(arg_ir, "")?,
            UnaryOperation::BitNot => builder.build_not(arg_ir, "")?,
        };

        Ok(IntegerValue {
            ir: result_ir,
            value_type: arg_type,
        }
        .into())
    }

    fn to_ir_expanded(
        &self,
        new_type: &IntegerType,
        expression_translator: &ExpressionTranslator<'ctx, '_, '_, '_>,
    ) -> CompilationResult<IntValue<'ctx>> {
        let builder = expression_translator.builder();
        let context = expression_translator.context();
        Ok(if new_type.is_signed {
            builder.build_int_s_extend(self.ir, new_type.to_ir(context), "")?
        } else {
            builder.build_int_z_extend(self.ir, new_type.to_ir(context), "")?
        })
    }
}
