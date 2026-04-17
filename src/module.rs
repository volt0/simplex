use crate::ast;
use crate::definition::Definition;
use crate::errors::CompilationResult;
use crate::function::Function;
use crate::module_translator::ModuleTranslator;
use crate::translator::Translator;

pub trait ModuleVisitor {
    fn visit_function(&mut self, name: Option<&str>, function: &Function) -> CompilationResult<()>;
}

pub struct Module {
    pub defs: Vec<Definition>,
}

impl Module {
    pub fn from_ast(translator: &Translator, module_ast: ast::Module) -> CompilationResult<Self> {
        let mut module = Self { defs: Vec::new() };
        let mut module_translator = ModuleTranslator::new(translator);
        for def in module_ast.defs {
            let def = Definition::from_ast(def)?;
            def.visit(&mut module_translator)?;
            module.defs.push(def);
        }

        module_translator.run_test();

        Ok(module)
    }
}
