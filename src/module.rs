use std::cell::RefCell;
use std::rc::Rc;

use crate::ast;
use crate::definitions::DefinitionBuilder;
use crate::function::Function;
use crate::namespace::Namespace;

pub trait ModuleVisitor {
    fn visit_function(&self, function: &Function);
}

pub struct Module {
    body: RefCell<Namespace>,
}

impl Module {
    pub fn new(module_ast: ast::Module) -> Rc<Self> {
        let module = Module {
            body: RefCell::default(),
        };

        let mut definition_builders: Vec<DefinitionBuilder> = vec![];
        for definition_ast in module_ast.definitions {
            let builder = module.create_definition(definition_ast);
            definition_builders.push(builder);
        }

        for builder in definition_builders.into_iter() {
            builder.build();
        }

        Rc::new(module)
    }

    pub fn visit(&self, visitor: &dyn ModuleVisitor) {
        self.body.borrow().visit(visitor);
    }
}

impl Module {
    fn create_definition(&self, definition_ast: ast::Definition) -> DefinitionBuilder {
        match definition_ast.value {
            ast::DefinitionValue::Function(function_ast) => {
                self.create_function(definition_ast.name, function_ast)
            }
        }
    }

    fn create_function(&self, name: String, function_ast: ast::Function) -> DefinitionBuilder {
        let function = Function::new(Some(name), &function_ast);
        self.body.borrow_mut().add_function(&function);
        DefinitionBuilder::define_function(&function, function_ast)
    }
}
