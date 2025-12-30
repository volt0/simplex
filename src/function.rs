use std::cell::OnceCell;
use std::rc::Rc;

use crate::ast;
use crate::basic_block::{BasicBlock, BasicBlockBuilder};
use crate::function_signature::FunctionSignature;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::types::{FunctionType, TypeSpec};

pub trait FunctionVisitor {
    fn visit_basic_block(&self, basic_block: &BasicBlock);
}

pub struct Function {
    name: Option<String>,
    signature: FunctionSignature,
    root_block: OnceCell<BasicBlock>,
}

impl Function {
    pub fn new(name: Option<String>, function_ast: &ast::Function) -> Rc<Self> {
        let signature_ast = &function_ast.signature;
        let signature = FunctionSignature::from_ast(signature_ast);

        Rc::new(Function {
            root_block: OnceCell::new(),
            name,
            signature,
        })
    }

    pub fn get_return_type(&self) -> TypeSpec {
        self.signature.return_type.clone()
    }

    pub fn get_function_type(&self) -> Box<FunctionType> {
        FunctionType::new(&self.signature)
    }

    pub fn get_mangled_name(&self) -> &str {
        self.name.as_deref().unwrap_or("<unknown>")
    }

    pub fn visit(&self, visitor: &dyn FunctionVisitor) {
        match self.root_block.get() {
            Some(root_basic_block) => visitor.visit_basic_block(root_basic_block),
            None => {
                todo!()
            }
        }
    }

    fn resolve_local(&self, name: &String) -> Option<LocalScopeItem> {
        for arg in &self.signature.args {
            let arg_ref = arg.as_ref();
            if arg_ref.name == *name {
                return Some(LocalScopeItem::Argument(arg.clone()));
            }
        }
        None
    }
}

pub struct FunctionBuilder {
    function: Rc<Function>,
    function_ast: ast::Function,
}

impl FunctionBuilder {
    pub fn new(function: &Rc<Function>, function_ast: ast::Function) -> Self {
        FunctionBuilder {
            function: function.clone(),
            function_ast,
        }
    }

    pub fn build(self) -> Rc<Function> {
        let body_ast = self.function_ast.body;
        let function = self.function;
        match body_ast {
            ast::FunctionBody::Forward => todo!(),
            ast::FunctionBody::BasicBlock(ast::BasicBlock {
                statements: statements_ast,
            }) => {
                let scope = FunctionScope {
                    function: function.clone(),
                };
                let basic_block_builder = BasicBlockBuilder::from_ast(statements_ast, &scope);
                let basic_block = basic_block_builder.build();
                _ = function.root_block.set(basic_block);
            }
        }

        function
    }
}

struct FunctionScope {
    function: Rc<Function>,
}

impl LocalScope for FunctionScope {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        self.function.resolve_local(name)
    }

    fn current_function(&self) -> Rc<Function> {
        self.function.clone()
    }
}
