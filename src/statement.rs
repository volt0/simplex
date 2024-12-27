use crate::ast;
use crate::expression::Expression;
use crate::type_spec::TypeSpec;
use slotmap::DefaultKey;
use std::cell::OnceCell;
use std::rc::Rc;

pub enum Statement {
    Let(Rc<Value>),
    Return(Box<Expression>),
}

impl Statement {
    pub fn from_ast(statement_ast: &ast::Statement) -> Self {
        match statement_ast {
            ast::Statement::Let(var) => {
                let value_type = var.value_type.as_ref().unwrap();
                let init_expression = var.init_expression.as_ref().unwrap();
                Statement::Let(Rc::new(Value {
                    type_spec: TypeSpec::from_ast(value_type),
                    assigned_exp: Expression::from_ast(init_expression),
                    ir_id: Default::default(),
                }))
            }
            ast::Statement::Return(exp) => {
                let exp = exp.as_ref().unwrap();
                Statement::Return(Expression::from_ast(exp))
            }
            _ => todo!(),
        }
    }
}

pub struct Value {
    pub type_spec: TypeSpec,
    pub assigned_exp: Box<Expression>,
    pub ir_id: OnceCell<DefaultKey>,
}

pub struct MutableValue {}

pub struct Variable {}
