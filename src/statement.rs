use crate::ast;
use crate::expression::ExpressionNode;
use crate::scope::LocalScope;
use crate::type_spec::TypeSpec;
use slotmap::DefaultKey;
use std::cell::OnceCell;
use std::rc::Rc;

pub enum Statement {
    Let(Rc<ValueAssignment>),
    Return(Box<ExpressionNode>),
}

impl Statement {
    pub fn from_ast(statement_ast: &ast::Statement, scope: &dyn LocalScope) -> Self {
        match statement_ast {
            ast::Statement::Let(var) => {
                let init_expression = var.init_expression.as_ref().unwrap();
                let value_type = var.value_type.as_ref().unwrap();
                let value = Rc::new(ValueAssignment {
                    name: var.name.clone(),
                    type_spec: TypeSpec::from_ast(value_type),
                    assigned_exp: ExpressionNode::from_ast(init_expression, scope),
                    ir_id: Default::default(),
                });
                Statement::Let(value)
            }
            ast::Statement::Return(exp) => {
                let exp = exp.as_ref().unwrap();
                Statement::Return(ExpressionNode::from_ast(exp, scope))
            }
            ast::Statement::BasicBlock(_) => todo!(),
            ast::Statement::Var(_) => todo!(),
            ast::Statement::If(_, _) => todo!(),
            ast::Statement::While(_, _) => todo!(),
            ast::Statement::For(_, _, _) => todo!(),
            ast::Statement::Break => todo!(),
            ast::Statement::Continue => todo!(),
            ast::Statement::Expression(_) => todo!(),
        }
    }
}

pub struct ValueAssignment {
    pub name: String,
    pub type_spec: TypeSpec,
    pub assigned_exp: Box<ExpressionNode>,
    pub ir_id: OnceCell<DefaultKey>,
}
