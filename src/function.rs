use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::ast;
use crate::basic_block::{BasicBlock, BasicBlockBuilder};
use crate::module::ModuleTranslator;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::StatementTranslator;
use crate::types::function::FunctionType;
use crate::types::TypeSpec;

pub trait FunctionVisitor {
    fn visit_basic_block(&self, basic_block: &BasicBlock);
}

pub struct Function {
    name: Option<String>,
    signature: FunctionSignature,
    root_block: OnceCell<BasicBlock>,
}

impl Function {
    pub fn from_ast(function_ast: &ast::Function, name: Option<String>) -> Rc<Self> {
        let signature_ast = &function_ast.signature;
        let signature = FunctionSignature::from_ast(signature_ast);

        Rc::new(Function {
            root_block: OnceCell::new(),
            name,
            signature,
        })
    }

    pub fn return_type(&self) -> TypeSpec {
        self.signature.return_type.clone()
    }

    pub fn function_type(&self) -> Box<FunctionType> {
        FunctionType::new(&self.signature)
    }

    pub fn mangled_name(&self) -> &str {
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
    pub fn from_ast(signature_ast: &ast::FunctionSignature) -> Self {
        let mut signature = FunctionSignature {
            return_type: TypeSpec::from_ast(&signature_ast.return_type.clone().unwrap()),
            args: vec![],
        };

        for arg_ast in &signature_ast.args {
            signature.create_argument(arg_ast);
        }

        signature
    }

    pub fn create_argument(&mut self, arg_ast: &ast::FunctionArgument) {
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
