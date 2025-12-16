use crate::function::FunctionSignature;
use crate::types::TypeSpec;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionType {
    pub arg_types: Vec<TypeSpec>,
    pub return_type: TypeSpec,
}

impl FunctionType {
    pub fn new(function_signature: &FunctionSignature) -> Box<Self> {
        let mut function_type = Box::new(FunctionType {
            arg_types: Vec::with_capacity(function_signature.args.len()),
            return_type: function_signature.return_type.clone(),
        });

        for arg in &function_signature.args {
            function_type.arg_types.push(arg.arg_type.clone())
        }

        function_type
    }
}
