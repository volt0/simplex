use crate::ast;
use crate::basic_block::BasicBlock;
use crate::errors::CompilationResult;
use crate::function_value::FunctionValue;

pub trait FunctionVisitor {
    fn visit_body(&self, body: &BasicBlock) -> CompilationResult<()>;
}

pub struct Function<'ctx> {
    pub body: BasicBlock,
    pub inner: FunctionValue<'ctx>,
}

impl<'ctx> Function<'ctx> {
    pub fn from_ast(
        func_ast: ast::Function,
        inner: FunctionValue<'ctx>,
    ) -> CompilationResult<Self> {
        Ok(Self::new(BasicBlock::from_ast(func_ast.body)?, inner))
    }

    pub fn new(body: BasicBlock, inner: FunctionValue<'ctx>) -> Self {
        Self { body, inner }
    }

    pub fn visit(&self, visitor: &dyn FunctionVisitor) -> CompilationResult<()> {
        visitor.visit_body(&self.body)
    }
}
