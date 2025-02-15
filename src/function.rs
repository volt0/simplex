use crate::ast;
use crate::basic_block::BasicBlock;
use crate::module::Module;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::types::Type;
use std::cell::OnceCell;
use std::rc::{Rc, Weak};

pub trait FunctionVisitor {
    fn visit_basic_block(&self, basic_block: Rc<BasicBlock>);
}

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

pub struct Function {
    args: Vec<Rc<FunctionArgument>>,
    return_type: Type,
    entry_basic_block: OnceCell<Rc<BasicBlock>>,
    weak_self: Weak<Function>,
}

impl LocalScope for Function {
    fn resolve(&self, name: &String) -> Option<LocalScopeItem> {
        for arg in &self.args {
            let arg_ref = arg.as_ref();
            if arg_ref.name == *name {
                return Some(LocalScopeItem::Argument(arg.clone()));
            }
        }
        None
    }

    fn current_function(&self) -> Rc<Function> {
        self.weak_self.upgrade().unwrap()
    }
}

impl Function {
    pub fn from_ast(signature: &ast::FunctionSignature, module: Rc<Module>) -> Rc<Self> {
        _ = module;

        let mut args = vec![];
        for (arg_id, arg_ast) in signature.args.iter().enumerate() {
            args.push(Rc::new(FunctionArgument {
                name: arg_ast.name.clone(),
                arg_type: Type::from_ast(&arg_ast.arg_type),
                pos_id: arg_id as u32,
            }));
        }

        let return_type = Type::from_ast(&signature.return_type.clone().unwrap());
        Rc::new_cyclic(|weak_self| Function {
            args,
            return_type,
            entry_basic_block: Default::default(),
            weak_self: weak_self.clone(),
        })
    }

    pub fn init_implementation(self: Rc<Self>, entry_basic_block_ast: &ast::BasicBlock) {
        let scope = self.clone() as Rc<dyn LocalScope>;
        self.entry_basic_block
            .set(BasicBlock::from_ast(
                &entry_basic_block_ast.statements,
                &scope,
            ))
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
        let entry_basic_block = self.entry_basic_block.get().unwrap();
        visitor.visit_basic_block(entry_basic_block.clone());
    }
}
