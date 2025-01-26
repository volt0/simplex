use crate::ast;
use crate::expression::{ExpressionCompiler, ExpressionEdge};
use crate::function::{Function, FunctionCompiler};
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::{Statement, ValueAssignment};
use inkwell::values::BasicValueEnum;
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};

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
}

impl LocalScope for BasicBlock {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        let inner = self.inner.borrow();
        if let Some(value) = inner.resolve_local(name) {
            return Some(value);
        }

        self.parent().resolve(name)
    }

    fn function(&self) -> Rc<Function> {
        self.parent().function()
    }
}

impl BasicBlock {
    pub fn compile(&self, basic_block_compiler: &BasicBlockCompiler) {
        let inner = self.inner.borrow();
        for stmt in inner.statements.iter() {
            basic_block_compiler.compile_statement(stmt);
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

#[repr(transparent)]
pub struct BasicBlockCompiler<'ctx, 'm, 'f> {
    parent: &'f FunctionCompiler<'ctx, 'm>,
}

impl<'ctx, 'm, 'f> Deref for BasicBlockCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionCompiler<'ctx, 'm>) -> Self {
        Self { parent }
    }

    fn compile_statement(&self, stmt: &Statement) {
        match stmt {
            Statement::ValueAssignment(var) => {
                self.compile_statement_let(var);
            }
            Statement::Return(exp) => {
                self.compile_statement_return(exp);
            }
        }
    }

    fn compile_expression(&self, exp: &ExpressionEdge) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionCompiler::new(self);
        exp_compiler.compile_expression(exp)
    }

    fn compile_statement_let(&self, val: &ValueAssignment) {
        let value = self.compile_expression(val.exp.as_ref());
        let value_id = self.store_value(value);
        val.ir_id.set(value_id).unwrap();
    }

    fn compile_statement_return(&self, exp: &ExpressionEdge) {
        let result = self.compile_expression(exp);
        self.builder().build_return(Some(&result)).unwrap();
    }
}
