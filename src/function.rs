use crate::ast;
use crate::basic_block::{BasicBlock, BasicBlockBuilder};
use crate::scope::{LocalScope, LocalScopeItem};
use crate::types::Type;
use std::cell::RefCell;
use std::rc::Rc;

pub trait FunctionVisitor {
    fn visit_basic_block(&self, basic_block: &BasicBlock);
}

struct FunctionInner {
    body_ast: Option<ast::FunctionBody>,
    root_block: Option<BasicBlock>,
}

pub struct Function {
    inner: RefCell<FunctionInner>,
    signature: Rc<FunctionSignature>,
}

impl Function {
    pub fn from_ast(function_ast: ast::Function) -> Rc<Self> {
        let ast::Function {
            signature: signature_ast,
            body: body_ast,
        } = function_ast;

        let mut signature = FunctionSignature {
            return_type: Type::from_ast(&signature_ast.return_type.clone().unwrap()),
            args: vec![],
        };

        for arg_ast in signature_ast.args {
            signature.create_argument(arg_ast);
        }

        Rc::new(Function {
            signature: Rc::new(signature),
            inner: RefCell::new(FunctionInner {
                body_ast: Some(body_ast),
                root_block: None,
            }),
        })
    }

    pub fn signature(&self) -> &Rc<FunctionSignature> {
        &self.signature
    }

    pub fn return_type(&self) -> Type {
        self.signature.return_type.clone()
    }

    pub fn is_complete(&self) -> bool {
        let inner = self.inner.borrow();
        inner.root_block.is_some()
    }

    pub fn traversal_pass(self: &Rc<Self>) {
        if !self.is_complete() {
            let mut inner = self.inner.borrow_mut();
            let body_ast = inner.body_ast.take().unwrap();
            match body_ast {
                ast::FunctionBody::Forward => todo!(),
                ast::FunctionBody::BasicBlock(ast::BasicBlock {
                    statements: statements_ast,
                }) => {
                    let scope = FunctionScope {
                        function: self.clone(),
                    };
                    let basic_block_builder = BasicBlockBuilder::from_ast(statements_ast, &scope);
                    let basic_block = basic_block_builder.build();
                    inner.root_block.replace(basic_block);
                }
            }
        }
    }

    pub fn visit(&self, visitor: &dyn FunctionVisitor) {
        let inner = self.inner.borrow();
        match &inner.root_block {
            Some(root_basic_block) => visitor.visit_basic_block(root_basic_block),
            None => {
                todo!()
            }
        }
    }
}

impl Function {
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

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionArgument {
    pub id: u32,
    pub name: String,
    pub arg_type: Type,
}

impl FunctionArgument {
    pub fn arg_type(&self) -> Type {
        self.arg_type.clone()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSignature {
    pub args: Vec<Rc<FunctionArgument>>,
    pub return_type: Type,
}

impl FunctionSignature {
    pub fn iter_args(&self) -> impl Iterator<Item = &Rc<FunctionArgument>> + use<'_> {
        let args = &self.args;
        args.iter()
    }

    pub fn create_argument(&mut self, arg_ast: ast::FunctionArgument) {
        let id = self.args.len() as u32;
        let name = arg_ast.name.clone();

        let arg_type = Type::from_ast(&arg_ast.arg_type);
        let arg = Rc::new(FunctionArgument { id, name, arg_type });
        self.args.push(arg);
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
