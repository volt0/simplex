use crate::float_type::FloatType;
use crate::integer_type::IntegerType;

pub enum Type {
    Integer(IntegerType),
    Float(FloatType),
    Bool,
}
