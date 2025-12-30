use crate::types::TypeSpec;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionArgument {
    pub id: u32,
    pub name: String,
    pub arg_type: TypeSpec,
}
