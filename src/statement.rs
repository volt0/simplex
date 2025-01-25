use crate::ast;
use crate::expression::ExpressionEdge;
use crate::scope::LocalScope;
use crate::types::{Type, TypeHint};
use slotmap::DefaultKey;
use std::cell::OnceCell;
use std::rc::Rc;

pub enum Statement {
    Let(Rc<ValueAssignment>),
    Return(Box<ExpressionEdge>),
}

impl Statement {
    pub fn from_ast(statement_ast: &ast::Statement, scope: &dyn LocalScope) -> Self {
        match statement_ast {
            ast::Statement::Let(var_ast) => Self::from_ast_let(var_ast, scope),
            ast::Statement::Return(exp) => {
                let exp = exp.as_ref().unwrap();
                Statement::Return(ExpressionEdge::from_ast(exp, scope))
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

    pub fn from_ast_let(var_ast: &ast::Variable, scope: &dyn LocalScope) -> Self {
        let init_expression_ast = var_ast.init_expression.as_ref().unwrap();
        let exp = ExpressionEdge::from_ast(init_expression_ast, scope);

        let type_spec_ast = var_ast.value_type.as_ref();
        let type_hint = TypeHint::from_type_spec(type_spec_ast);
        let type_spec = exp.get_or_infer_type(&type_hint);

        Statement::Let(Rc::new(ValueAssignment {
            name: var_ast.name.clone(),
            type_spec,
            assigned_exp: exp,
            ir_id: Default::default(),
        }))
    }

    pub fn from_ast_return(exp_ast: &ast::Expression, scope: &dyn LocalScope) -> Self {
        let exp = ExpressionEdge::from_ast(exp_ast, scope);
        todo!()
        // let function = scope.function();
        // let type_hint = TypeHint::Explicit(function.return_type());
        // exp.get_or_infer_type(&type_hint);
        // Statement::Return(exp)
    }
}

#[derive(Debug)]
pub struct ValueAssignment {
    pub name: String,
    pub type_spec: Type,
    pub assigned_exp: Box<ExpressionEdge>,
    pub ir_id: OnceCell<DefaultKey>,
}
