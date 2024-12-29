use crate::ast;
use crate::expression::{Expression, ExpressionCompiler};
use crate::function::FunctionCompiler;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::{Statement, ValueAssignment};
use inkwell::values::BasicValueEnum;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct BasicBlock {
    inner: RefCell<BasicBlockInner>,
}

impl BasicBlock {
    pub fn from_ast(basic_block_ast: &Vec<ast::Statement>, parent: Rc<dyn LocalScope>) -> Rc<Self> {
        let basic_block = Rc::new(BasicBlock {
            inner: RefCell::new(BasicBlockInner {
                statements: vec![],
                parent_scope: Rc::downgrade(&parent),
            }),
        });

        for statement_ast in basic_block_ast {
            let statement = Statement::from_ast(statement_ast, basic_block.as_ref());
            let mut inner = basic_block.inner.borrow_mut();
            inner.statements.push(statement);
        }
        basic_block.clone()
    }
}

impl LocalScope for BasicBlock {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        let inner = self.inner.borrow();
        for statement in inner.statements.iter() {
            match statement {
                Statement::Let(val) => {
                    let val = val.clone();
                    if val.name == *name {
                        return Some(LocalScopeItem::Value(val));
                    }
                }
                _ => (),
            }
        }

        let parent = inner.parent_scope.upgrade().unwrap();
        parent.resolve(name)
    }
}

pub struct BasicBlockInner {
    statements: Vec<Statement>,
    parent_scope: Weak<dyn LocalScope>,
}

#[repr(transparent)]
pub struct BasicBlockCompiler<'ctx, 'm, 'f> {
    function_compiler: &'f FunctionCompiler<'ctx, 'm>,
}

impl<'ctx, 'm, 'f> Deref for BasicBlockCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.function_compiler
    }
}

impl<'ctx, 'm, 'f> BasicBlockCompiler<'ctx, 'm, 'f> {
    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionCompiler::new(self);
        exp_compiler.compile_expression(exp)
    }

    pub fn compile_statement_let(&self, val: &ValueAssignment) {
        let value = self.compile_expression(val.assigned_exp.as_ref());
        let value_id = self.store_value(value);
        val.ir_id.set(value_id).unwrap();
    }

    pub fn compile_statement_return(&self, exp: &Expression) {
        let result = self.compile_expression(exp);
        self.builder().build_return(Some(&result)).unwrap();
    }
}

impl BasicBlock {
    pub fn compile(&self, function_compiler: &FunctionCompiler) {
        let basic_block_compiler = BasicBlockCompiler { function_compiler };
        let inner = self.inner.borrow();
        for statement in inner.statements.iter() {
            match statement {
                Statement::Let(var) => {
                    basic_block_compiler.compile_statement_let(var.as_ref());
                }
                Statement::Return(exp) => {
                    basic_block_compiler.compile_statement_return(exp);
                }
            }
        }
    }
}
