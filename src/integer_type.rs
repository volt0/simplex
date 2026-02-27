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
