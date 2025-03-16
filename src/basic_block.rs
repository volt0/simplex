use crate::ast;
use crate::function::Function;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::Statement;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::{Rc, Weak};

pub trait BasicBlockVisitor {
    fn visit_statement(&self, stmt: &Statement);
}

pub struct BasicBlock {
    inner: RefCell<BasicBlockInner>,
    function: Weak<Function>,
}

impl BasicBlock {
    pub fn from_ast(
        basic_block_ast: &Vec<ast::Statement>,
        function: Weak<Function>,
        parent: &dyn LocalScope,
    ) -> Self {
        let basic_block = BasicBlock {
            inner: RefCell::new(BasicBlockInner {
                statements: vec![],
                locals: Default::default(),
            }),
            function,
        };

        let scope = BasicBlockScope {
            basic_block: &basic_block,
            parent,
        };

        for statement_ast in basic_block_ast {
            let statement = Statement::from_ast(statement_ast, &scope);
            let mut inner = basic_block.inner.borrow_mut();
            inner.add_statement(statement);
        }

        basic_block
    }

    pub fn traversal(&self, visitor: &dyn BasicBlockVisitor) {
        let inner = self.inner.borrow();
        for stmt in inner.statements.iter() {
            visitor.visit_statement(stmt);
        }
    }
}

pub struct BasicBlockInner {
    statements: Vec<Statement>,
    locals: HashMap<String, LocalScopeItem>,
}

impl BasicBlockInner {
    fn add_statement(&mut self, statement: Statement) {
        match &statement {
            Statement::ValueAssignment(value) => {
                self.locals.insert(value.name.clone(), value.into());
            }
            _ => (),
        }

        self.statements.push(statement);
    }

    fn resolve_local(&self, name: &String) -> Option<LocalScopeItem> {
        self.locals.get(name).cloned()
    }
}

struct BasicBlockScope<'b, 'p> {
    basic_block: &'b BasicBlock,
    parent: &'p dyn LocalScope,
}

impl LocalScope for BasicBlockScope<'_, '_> {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        let inner = self.basic_block.inner.borrow();
        if let Some(value) = inner.resolve_local(name) {
            return Some(value);
        }

        self.parent.resolve(name)
    }

    fn current_function(&self) -> Rc<Function> {
        self.basic_block
            .function
            .upgrade()
            .expect("function dropped")
    }
}
