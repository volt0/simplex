pub type TypeHint = Option<TypeSpec>;

#[derive(Clone, Debug, PartialEq)]
pub enum TypeSpec {
    Boolean,
    Integer(IntegerType),
    Float(FloatType),
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum IntegerTypeSize {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct IntegerType {
    pub is_signed: bool,
    pub width: IntegerTypeSize,
}

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub enum FloatType {
    F32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType {
    pub arg_types: Vec<TypeSpec>,
    pub return_type: TypeSpec,
}
