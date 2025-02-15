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
    parent: Weak<dyn LocalScope>,
}

impl BasicBlock {
    pub fn from_ast(
        basic_block_ast: &Vec<ast::Statement>,
        parent: &Rc<dyn LocalScope>,
    ) -> Rc<Self> {
        let basic_block = Rc::new(BasicBlock {
            inner: RefCell::new(BasicBlockInner {
                statements: vec![],
                locals: Default::default(),
            }),
            parent: Rc::downgrade(parent),
        });

        for statement_ast in basic_block_ast {
            let statement = Statement::from_ast(statement_ast, basic_block.as_ref());
            basic_block.add_statement(statement);
        }

        basic_block
    }

    pub fn parent(&self) -> Rc<dyn LocalScope> {
        self.parent.upgrade().unwrap()
    }

    fn add_statement(&self, statement: Statement) {
        let mut inner = self.inner.borrow_mut();
        inner.add_statement(statement);
    }

    pub fn traversal(&self, visitor: &dyn BasicBlockVisitor) {
        let inner = self.inner.borrow();
        for stmt in inner.statements.iter() {
            visitor.visit_statement(stmt);
        }
    }
}

impl LocalScope for BasicBlock {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        let inner = self.inner.borrow();
        if let Some(value) = inner.resolve_local(name) {
            return Some(value);
        }

        self.parent().resolve(name)
    }

    fn current_function(&self) -> Rc<Function> {
        self.parent().current_function()
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
