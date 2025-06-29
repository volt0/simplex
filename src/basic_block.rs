use crate::ast;
use crate::expression::{Expression, ExpressionCompiler};
use crate::function::Function;
use crate::function::FunctionCompiler;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::{Statement, ValueAssignment};

use inkwell::values::BasicValueEnum;
use std::collections::HashMap;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct BasicBlockBuilder {
    inner: BasicBlock,
}

impl BasicBlockBuilder {
    pub fn from_ast(
        statements_ast: impl IntoIterator<Item = ast::Statement>,
        parent_scope: &dyn LocalScope,
    ) -> Self {
        let mut builder = BasicBlockBuilder {
            inner: BasicBlock {
                statements: vec![],
                locals: HashMap::default(),
                function: Rc::downgrade(&parent_scope.current_function()),
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

impl<'ctx, 'm, 'f> BasicBlockVisitor for BasicBlockCompiler<'ctx, 'm, 'f> {
    fn visit_statement(&self, stmt: &Statement) {
        match stmt {
            Statement::ValueAssignment(var) => {
                self.compile_statement_let(var);
            }
            Statement::Return(exp) => {
                self.compile_statement_return(exp);
            }
        }
    }
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionCompiler<'ctx, 'm>) -> Self {
        Self { parent }
    }

    fn compile_statement_let(&self, val: &ValueAssignment) {
        let value = self.compile_expression(val.exp.as_ref());
        let value_id = self.store_value(value);
        val.ir_id.set(value_id).unwrap();
    }

    fn compile_statement_return(&self, exp: &Expression) {
        let result = self.compile_expression(exp);
        self.builder().build_return(Some(&result)).unwrap();
    }

    fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionCompiler::new(self);
        exp_compiler.compile_expression(exp)
    }
}
