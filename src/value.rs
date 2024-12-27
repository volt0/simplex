use inkwell::values::IntValue;

#[derive(Clone)]
pub enum Value<'ctx> {
    Integer(IntValue<'ctx>),
}
