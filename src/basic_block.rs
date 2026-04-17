use crate::ast;
use crate::errors::CompilationResult;
use crate::statement::{Statement, StatementVisitor};

pub struct BasicBlock {
    statements: Vec<Statement>,
}

impl BasicBlock {
    pub fn from_ast(block_ast: ast::BasicBlock) -> CompilationResult<Self> {
        Ok(Self {
            statements: block_ast.statements.into_iter().collect::<Vec<Statement>>(),
        })
    }

    pub fn new(statements: Vec<Statement>) -> Self {
        Self { statements }
    }

    pub fn visit(&self, visitor: &dyn StatementVisitor) -> CompilationResult<()> {
        for stmt in self.statements.iter() {
            stmt.visit(visitor)?;
        }
        Ok(())
    }
}
