use crate::ast;
use crate::basic_block::{BasicBlock, BasicBlockBuilder};
use crate::scope::{LocalScope, LocalScopeItem};
use crate::types::Type;
use std::cell::OnceCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct FunctionArgument {
    pub name: String,
    pub arg_type: Type,
    pub pos_id: u32,
}

impl FunctionArgument {
    pub fn arg_type(&self) -> Type {
        self.arg_type.clone()
    }
}

pub struct FunctionBuilder {
    inner: Function,
}

impl FunctionBuilder {
    pub fn from_ast(signature: ast::FunctionSignature) -> Self {
        let return_type = Type::from_ast(&signature.return_type.clone().unwrap());
        let mut builder = FunctionBuilder {
            inner: Function {
                return_type,
                args: vec![],
                body: OnceCell::default(),
            },
        };

        for arg_ast in signature.args {
            let name = arg_ast.name.clone();
            let arg_type = Type::from_ast(&arg_ast.arg_type);
            builder.add_argument(name, arg_type);
        }

        builder
    }

    pub fn add_argument(&mut self, name: String, arg_type: Type) {
        let args = &mut self.inner.args;
        let arg_id = args.len() as u32;
        args.push(Rc::new(FunctionArgument {
            name,
            arg_type,
            pos_id: arg_id,
        }));
    }

    pub fn build(self) -> Rc<Function> {
        Rc::new(self.inner)
    }
}

pub trait FunctionVisitor {
    fn visit_basic_block(&self, basic_block: &BasicBlock);
}

pub struct Function {
    args: Vec<Rc<FunctionArgument>>,
    return_type: Type,
    body: OnceCell<FunctionBody>,
}

impl Function {
    pub fn init_implementation(self: Rc<Self>, body_ast: ast::BasicBlock) {
        let scope = FunctionScope {
            function: self.clone(),
        };

        let basic_block_builder = BasicBlockBuilder::from_ast(body_ast.statements, &self, &scope);

        let basic_block = basic_block_builder.build();
        self.body
            .set(FunctionBody::RootBasicBlock(basic_block))
            .ok()
            .unwrap();
    }

    pub fn iter_args(&self) -> impl Iterator<Item = &Rc<FunctionArgument>> + use<'_> {
        self.args.iter()
    }

    pub fn return_type(&self) -> Type {
        self.return_type.clone()
    }

    pub fn traversal(&self, visitor: &dyn FunctionVisitor) {
        let body = self.body.get().unwrap();
        match body {
            FunctionBody::RootBasicBlock(root_basic_block) => {
                visitor.visit_basic_block(root_basic_block)
            }
        }
    }

    fn resolve_local(&self, name: &String) -> Option<LocalScopeItem> {
        for arg in &self.args {
            let arg_ref = arg.as_ref();
            if arg_ref.name == *name {
                return Some(LocalScopeItem::Argument(arg.clone()));
            }
        }
        None
    }
}

pub enum FunctionBody {
    RootBasicBlock(BasicBlock),
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
