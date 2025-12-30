use std::rc::Rc;

use crate::ast;
use crate::function_argument::FunctionArgument;
use crate::types::TypeSpec;

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSignature {
    pub args: Vec<Rc<FunctionArgument>>,
    pub return_type: TypeSpec,
}

impl FunctionSignature {
    pub fn from_ast(signature_ast: &ast::FunctionSignature) -> Self {
        let mut signature = FunctionSignature {
            return_type: TypeSpec::from_ast(&signature_ast.return_type.clone().unwrap()),
            args: vec![],
        };

        for arg_ast in &signature_ast.args {
            signature.create_argument(arg_ast);
        }

        signature
    }

    pub fn create_argument(&mut self, arg_ast: &ast::FunctionArgument) {
        let id = self.args.len() as u32;
        let name = arg_ast.name.clone();

        let arg_type = TypeSpec::from_ast(&arg_ast.arg_type);
        let arg = Rc::new(FunctionArgument { id, name, arg_type });
        self.args.push(arg);
    }
}
