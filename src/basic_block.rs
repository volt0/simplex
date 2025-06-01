use crate::ast;
use crate::function::Function;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::Statement;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub struct BasicBlockBuilder {
    inner: BasicBlock,
}

impl BasicBlockBuilder {
    pub fn from_ast(
        statements_ast: impl IntoIterator<Item = ast::Statement>,
        function: &Rc<Function>,
        parent_scope: &dyn LocalScope,
    ) -> Self {
        let mut builder = BasicBlockBuilder {
            inner: BasicBlock {
                statements: vec![],
                locals: HashMap::default(),
                function: Rc::downgrade(function),
            },
        };

        for statement_ast in statements_ast {
            let scope = BasicBlockScope {
                basic_block: &builder.inner,
                parent_scope,
            };

            let statement = Statement::from_ast(statement_ast, &scope);
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
    pub fn traversal(&self, visitor: &dyn BasicBlockVisitor) {
        for stmt in self.statements.iter() {
            visitor.visit_statement(stmt);
        }
    }

    fn resolve_local(&self, name: &String) -> Option<LocalScopeItem> {
        self.locals.get(name).cloned()
    }
}

struct BasicBlockScope<'b, 'p> {
    basic_block: &'b BasicBlock,
    parent_scope: &'p dyn LocalScope,
}

impl LocalScope for BasicBlockScope<'_, '_> {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        if let Some(value) = self.basic_block.resolve_local(name) {
            return Some(value);
        }

        self.parent_scope.resolve(name)
    }

    fn current_function(&self) -> Rc<Function> {
        self.basic_block
            .function
            .upgrade()
            .expect("function dropped")
    }
}
