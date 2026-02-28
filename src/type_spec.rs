use crate::integer_type::IntegerType;

pub type TypeHint = Option<TypeSpec>;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSpec {
    Void,
    Boolean,
    Integer(IntegerType),
    Float,
}
