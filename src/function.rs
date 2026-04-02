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
    pub signature: FunctionSignature,
    pub body: BasicBlock,
}

impl Function {
    pub fn new(signature: FunctionSignature, body: BasicBlock) -> Self {
        Function { signature, body }
    }

    pub fn visit(&self, visitor: &dyn FunctionVisitor) -> CompilationResult<()> {
        visitor.visit_body(&self.body)
    }
}
