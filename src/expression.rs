use crate::ast;
use crate::instruction::Instruction;
use crate::scope::LocalScope;
use crate::types::{TypeHint, TypeSpec};

pub struct Expression {
    exp_type: TypeSpec,
    instruction: Instruction,
}

impl Expression {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        type_hint: &TypeHint,
        scope: &dyn LocalScope,
    ) -> Box<Self> {
        match type_hint {
            None => todo!(),
            Some(exp_type) => Box::new(Expression {
                exp_type: exp_type.clone(),
                instruction: Instruction::from_ast(exp_ast, type_hint, scope),
            }),
        }
    }

    pub fn get_type(&self) -> TypeSpec {
        self.exp_type.clone()
    }

    pub fn get_instruction(&self) -> &Instruction {
        &self.instruction
    }
}
