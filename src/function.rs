use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::types::Type;

pub trait FunctionVisitor {
    fn visit_body(&self, basic_block: &BasicBlock) -> CompilationResult<()>;
}

pub struct FunctionSignature {
    pub args: Vec<FunctionArgument>,
    pub return_type: Type,
}

pub struct FunctionArgument {
    pub name: String,
    pub value_type: Type,
}

pub struct Function {
    pub name: Option<String>,
    pub signature: FunctionSignature,
    pub body: BasicBlock,
}

impl Function {
    pub fn new(name: Option<String>, signature: FunctionSignature, body: BasicBlock) -> Self {
        Function {
            name,
            signature,
            body,
        }
    }

    pub fn visit(&self, visitor: &dyn FunctionVisitor) -> CompilationResult<()> {
        visitor.visit_body(&self.body)
    }
}
