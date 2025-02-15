use crate::ast;
use crate::expression::ExpressionEdge;
use crate::scope::LocalScope;
use crate::types::{Type, TypeHint};
use slotmap::DefaultKey;
use std::cell::OnceCell;
use std::rc::Rc;

#[derive(Debug)]
pub enum Statement {
    ValueAssignment(Rc<ValueAssignment>),
    Return(Box<ExpressionEdge>),
}

impl Statement {
    pub fn from_ast(statement_ast: &ast::Statement, scope: &dyn LocalScope) -> Self {
        match statement_ast {
            ast::Statement::Let(var_ast) => Self::from_ast_let(var_ast, scope),
            ast::Statement::Return(exp) => match exp {
                None => todo!(),
                Some(exp) => Self::from_ast_return(exp, scope),
            },
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

    pub fn from_ast_let(var_ast: &ast::Variable, scope: &dyn LocalScope) -> Self {
        let type_spec_ast = var_ast.value_type.as_ref();
        let type_hint = TypeHint::from_type_spec(type_spec_ast);

        let exp_ast = var_ast.init_expression.as_ref().unwrap();
        let exp = ExpressionEdge::from_ast(exp_ast, scope, &type_hint);
        Statement::ValueAssignment(ValueAssignment::new(var_ast.name.clone(), exp))
    }

    pub fn from_ast_return(exp_ast: &ast::Expression, scope: &dyn LocalScope) -> Self {
        let function = scope.current_function();
        let type_hint = TypeHint::Explicit(function.return_type());
        let exp = ExpressionEdge::from_ast(exp_ast, scope, &type_hint);
        Statement::Return(exp)
    }
}

#[derive(Debug)]
pub struct ValueAssignment {
    pub name: String,
    pub type_spec: Type,
    pub exp: Box<ExpressionEdge>,
    pub ir_id: OnceCell<DefaultKey>,
}

impl ValueAssignment {
    pub fn new(name: String, exp: Box<ExpressionEdge>) -> Rc<Self> {
        Rc::new(ValueAssignment {
            type_spec: exp.exp_type(),
            name,
            exp,
            ir_id: Default::default(),
        })
    }
}
