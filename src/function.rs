use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::types::Type;

pub trait FunctionVisitor {
    fn visit_body(&self, body: &BasicBlock) -> CompilationResult<()>;
}

#[derive(Clone)]
pub struct FunctionSignature {
    pub args: Vec<FunctionArgument>,
    pub return_type: Type,
}

#[derive(Clone)]
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

    #[inline(always)]
    pub fn signature(&self) -> &FunctionSignature {
        &self.signature
    }

    pub fn visit(&self, visitor: &dyn FunctionVisitor) -> CompilationResult<()> {
        visitor.visit_body(&self.body)
    }
}
