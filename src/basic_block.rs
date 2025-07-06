use crate::ast;
use crate::function::Function;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::Statement;

use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct BasicBlockBuilder<'p> {
    inner: BasicBlock,
    parent_scope: &'p dyn LocalScope,
}

impl<'p> LocalScope for BasicBlockBuilder<'p> {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        if let Some(value) = self.inner.resolve_local(name) {
            return Some(value);
        }

        self.parent_scope.resolve(name)
    }

    fn current_function(&self) -> Rc<Function> {
        self.inner.function.upgrade().expect("function dropped")
    }
}

impl<'p> BasicBlockBuilder<'p> {
    pub fn new(parent_scope: &'p dyn LocalScope) -> Self {
        BasicBlockBuilder {
            inner: BasicBlock {
                statements: vec![],
                locals: HashMap::default(),
                function: Rc::downgrade(&parent_scope.current_function()),
            },
            parent_scope,
        }
    }

    pub fn from_ast(
        statements_ast: impl IntoIterator<Item = ast::Statement>,
        parent_scope: &'p dyn LocalScope,
    ) -> Self {
        let mut builder = BasicBlockBuilder::new(parent_scope);

        for statement_ast in statements_ast {
            let statement = Statement::from_ast(statement_ast, &builder);
            builder.add_statement(statement);
        }

        builder
    }

    pub fn add_statement(&mut self, statement: Statement) {
        match &statement {
            Statement::ValueAssignment(value) => {
                self.inner.locals.insert(value.name.clone(), value.into());
            }
            _ => (),
        }

        self.inner.statements.push(statement);
    }

    pub fn build(self) -> BasicBlock {
        self.inner
    }
}

pub trait BasicBlockVisitor {
    fn visit_statement(&self, stmt: &Statement);
}

pub struct BasicBlock {
    statements: Vec<Statement>,
    locals: HashMap<String, LocalScopeItem>,
    function: Weak<Function>,
}

impl BasicBlock {
    pub fn visit(&self, visitor: &dyn BasicBlockVisitor) {
        for stmt in self.statements.iter() {
            visitor.visit_statement(stmt);
        }
    }
}

impl BasicBlock {
    fn resolve_local(&self, name: &String) -> Option<LocalScopeItem> {
        self.locals.get(name).cloned()
    }
}
