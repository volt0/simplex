use crate::ast;
use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::types::TypeSpec;

pub trait FunctionVisitor {
    fn visit_body(&self, body: &BasicBlock) -> CompilationResult<()>;
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
    pub fn from_ast(func_ast: ast::Function) -> CompilationResult<Self> {
        let signature_ast = &func_ast.signature;
        let signature = FunctionSignature {
            args: signature_ast
                .args
                .iter()
                .map(|arg| FunctionArgument {
                    name: arg.name.clone(),
                    value_type: arg.value_type.clone(),
                })
                .collect(),
            return_type: signature_ast.return_type.clone(),
        };

        Ok(Self::new(signature, BasicBlock::from_ast(func_ast.body)?))
    }

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
