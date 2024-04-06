use inkwell::values::{BasicValue, BasicValueEnum, IntValue};

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
                sign_extend: false,
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

// impl<'ctx> Deref for Value<'ctx> {
//     type Target = dyn BasicValue<'ctx>;
//
//     fn deref(&'ctx self) -> &'ctx Self::Target {
//         match self {
//             Value::Integer(ref inner) => &inner.ir,
//         }
//     }
// }

#[derive(Clone)]
pub struct IntegerValue<'ctx> {
    pub ir: IntValue<'ctx>,
    pub sign_extend: bool,
}

// #[derive(Clone)]
// pub struct Value<'ctx> {
//     pub ir: BasicValueEnum<'ctx>,
//     pub sign_extend: bool,
// }
//
// impl<'ctx> Value<'ctx> {
//     pub fn from_ir(ir: BasicValueEnum<'ctx>) -> Self {
//         Value {
//             ir,
//             sign_extend: false,
//         }
//     }
//
//     pub fn new_integer(ir: IntValue<'ctx>, sign_extend: bool) -> Self {
//         Value {
//             ir: ir.into(),
//             sign_extend,
//         }
//     }
// }
//
// impl<'ctx> Deref for Value<'ctx> {
//     type Target = BasicValueEnum<'ctx>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.ir
//     }
// }
