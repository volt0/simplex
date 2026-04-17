use crate::ast;
use crate::errors::CompilationResult;
use crate::function::Function;
use crate::module::ModuleVisitor;

pub struct Definition {
    name: String,
    value: DefinitionValue,
}

pub enum DefinitionValue {
    Function(Function),
}

impl Definition {
    pub fn from_ast(def_ast: ast::Definition) -> CompilationResult<Self> {
        match def_ast.value {
            ast::DefinitionValue::Function(func) => Ok(Definition {
                name: def_ast.name,
                value: DefinitionValue::Function(Function::from_ast(func)?),
            }),
        }
    }

    pub fn visit(&self, visitor: &mut dyn ModuleVisitor) -> CompilationResult<()> {
        match &self.value {
            DefinitionValue::Function(func) => {
                visitor.visit_function(Some(self.name.as_str()), func)
            }
        }
    }
}
