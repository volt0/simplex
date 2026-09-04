use crate::ast;
use crate::errors::CompilationResult;
use crate::function::Function;
use crate::module::ModuleBuilder;

pub enum Definition<'ctx> {
    Function(Function<'ctx>),
}

impl<'ctx> Definition<'ctx> {
    pub fn new_from_ast(
        def_ast: ast::Definition,
        module_builder: &mut ModuleBuilder<'ctx>,
    ) -> CompilationResult<Self> {
        Ok(match def_ast.value {
            ast::DefinitionValue::Function(func_ast) => {
                Definition::Function(module_builder.create_function(
                    def_ast.name.as_str(),
                    func_ast.signature,
                    func_ast.body,
                )?)
            }
        })
    }
}
