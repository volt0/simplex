use crate::ast;
use crate::integer_expression::IntegerExpression;
use crate::scope::LocalScope;
use crate::types::{TypeHint, TypeSpec};

pub enum Expression {
    Integer(IntegerExpression),
}

impl Expression {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        type_hint: &TypeHint,
        scope: &dyn LocalScope,
    ) -> Box<Self> {
        match exp_ast {
            ast::Expression::Constant(_) => todo!(),
            ast::Expression::Identifier(_) => todo!(),
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::BinaryOperation(_) => todo!(),
            ast::Expression::UnaryOperation(_) => todo!(),
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
        }

        // match type_hint {
        //     None => todo!(),
        //     Some(exp_type) => match exp_type {
        //         TypeSpec::Void => todo!(),
        //         TypeSpec::Bool => todo!(),
        //         TypeSpec::Integer(exp_type) => {
        //             todo!()
        //         }
        //         TypeSpec::Float(_) => todo!(),
        //     },
        // }

        // match type_hint {
        //     None => todo!(),
        //     Some(exp_type) => Box::new(Expression {
        //         exp_type: exp_type.clone(),
        //         instruction: Instruction::from_ast(exp_ast, type_hint, scope),
        //     }),
        // }
    }

    pub fn get_type(&self) -> TypeSpec {
        todo!()
        // self.exp_type.clone()
    }
}
