use crate::ast::FunctionArgument;
use inkwell::values::BasicValueEnum;

#[derive(Clone)]
pub enum Identifier<'ctx> {
    // Value(Box<dyn Value<'ctx>>),
    Value(Value<'ctx>),
}

impl<'ctx> Identifier<'ctx> {
    pub fn new_argument(arg: FunctionArgument, ir: BasicValueEnum<'ctx>) -> Self {
        Identifier::Value(Value { ir })
    }
}

// pub trait Value<'ctx> {}

#[derive(Clone)]
pub struct Value<'ctx> {
    pub ir: BasicValueEnum<'ctx>,
    // pub value_type: Type<'ctx>,
}
