use crate::ast;

pub enum Constant {
    Integer(i32),
}

impl Constant {
    pub fn from_ast(const_ast: &ast::Constant) -> Constant {
        match const_ast {
            ast::Constant::Void => todo!(),
            ast::Constant::True => todo!(),
            ast::Constant::False => todo!(),
            ast::Constant::Integer(val) => Constant::Integer(*val),
            ast::Constant::Float(_) => todo!(),
            ast::Constant::String(_) => todo!(),
        }
    }
}
