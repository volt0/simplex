use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::ast;
use crate::basic_block::{BasicBlock, BasicBlockBuilder};
use crate::module::ModuleTranslator;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::StatementTranslator;
use crate::types::{FunctionType, TypeSpec};

pub trait FunctionVisitor {
    fn visit_basic_block(&self, basic_block: &BasicBlock);
}

pub struct Function {
    body_ast: RefCell<Option<ast::FunctionBody>>,
    root_block: RefCell<Option<BasicBlock>>,
    signature: FunctionSignature,
}

impl Function {
    pub fn from_ast(function_ast: ast::Function) -> Rc<Self> {
        let ast::Function {
            signature: signature_ast,
            body: body_ast,
        } = function_ast;

        let mut signature = FunctionSignature {
            return_type: TypeSpec::from_ast(&signature_ast.return_type.clone().unwrap()),
            args: vec![],
        };

        for arg_ast in signature_ast.args {
            signature.create_argument(arg_ast);
        }

        Rc::new(Function {
            body_ast: RefCell::new(Some(body_ast)),
            root_block: RefCell::new(None),
            signature,
        })
    }

    pub fn return_type(&self) -> TypeSpec {
        self.signature.return_type.clone()
    }

    pub fn function_type(&self) -> Box<FunctionType> {
        FunctionType::new(&self.signature)
    }

    pub fn is_complete(&self) -> bool {
        self.root_block.borrow().is_some()
    }

    pub fn traversal_pass(self: &Rc<Self>) {
        if !self.is_complete() {
            let body_ast = self.body_ast.take().unwrap();
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
                    self.root_block.replace(Some(basic_block));
                }
            }
        }
    }

    pub fn visit(&self, visitor: &dyn FunctionVisitor) {
        match self.root_block.borrow().as_ref() {
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
    pub arg_type: TypeSpec,
}

#[derive(Clone, Debug, PartialEq)]
pub struct FunctionSignature {
    pub args: Vec<Rc<FunctionArgument>>,
    pub return_type: TypeSpec,
}

impl FunctionSignature {
    pub fn create_argument(&mut self, arg_ast: ast::FunctionArgument) {
        let id = self.args.len() as u32;
        let name = arg_ast.name.clone();

        let arg_type = TypeSpec::from_ast(&arg_ast.arg_type);
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

pub struct FunctionTranslator<'ctx, 'm> {
    pub function_ir: FunctionValue<'ctx>,
    pub builder: Builder<'ctx>,
    parent: &'m ModuleTranslator<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionTranslator<'ctx, 'm> {
    type Target = ModuleTranslator<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm> FunctionVisitor for FunctionTranslator<'ctx, 'm> {
    fn visit_basic_block(&self, basic_block: &BasicBlock) {
        let basic_block_ir = self.context.append_basic_block(self.function_ir, "");
        self.builder.position_at_end(basic_block_ir);

        let translator = StatementTranslator::new(self);
        basic_block.visit(&translator);
    }
}

impl<'ctx, 'm> FunctionTranslator<'ctx, 'm> {
    pub fn new(parent: &'m ModuleTranslator<'ctx>, ir: FunctionValue<'ctx>) -> Self {
        let context = parent.context;
        let builder = context.create_builder();
        FunctionTranslator {
            parent,
            builder,
            function_ir: ir,
        }
    }

    pub fn load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.function_ir.get_nth_param(arg.id).unwrap()
    }
}
