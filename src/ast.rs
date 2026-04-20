pub use crate::basic_block::BasicBlock;
pub use crate::constant::Constant;
pub use crate::expression::Expression;
pub use crate::statement::Statement;
pub use crate::types::TypeSpec;

pub struct Module {
    pub defs: Vec<Definition>,
}

impl Module {
    pub fn new(defs: Vec<Definition>) -> Module {
        Module { defs }
    }
}

pub struct Definition {
    pub name: String,
    pub value: DefinitionValue,
}

impl Definition {
    pub fn define_function(name: String, function: Function) -> Self {
        Definition {
            name,
            value: DefinitionValue::Function(function),
        }
    }
}

pub enum DefinitionValue {
    Function(Function),
}

#[derive(Clone)]
pub struct FunctionSignature {
    pub args: Vec<FunctionArgument>,
    pub return_type: TypeSpec,
}

#[derive(Clone)]
pub struct FunctionArgument {
    pub name: String,
    pub value_type: TypeSpec,
}

pub struct Function {
    pub signature: FunctionSignature,
    pub body: BasicBlock,
}

impl Function {
    pub fn new(signature: FunctionSignature, body: BasicBlock) -> Self {
        Function { signature, body }
    }
}
