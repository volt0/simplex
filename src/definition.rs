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
    pub fn define_function(name: String, function: Function) -> Self {
        Definition {
            name,
            value: DefinitionValue::Function(function),
        }
    }

    pub fn visit(&self, visitor: &dyn ModuleVisitor) -> CompilationResult<()> {
        match &self.value {
            DefinitionValue::Function(func) => {
                visitor.visit_function(Some(self.name.as_str()), func)
            }
        }
    }
}
