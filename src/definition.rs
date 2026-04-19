use crate::function::Function;

pub enum Definition<'ctx> {
    Function(Function<'ctx>),
}
