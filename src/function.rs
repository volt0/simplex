use crate::ast;
use crate::basic_block::{BasicBlock, BasicBlockCompiler};
use crate::module::{Module, ModuleCompiler};
use crate::scope::{LocalScope, LocalScopeItem};
use crate::types::Type;
use inkwell::builder::Builder;
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

#[derive(Debug)]
pub struct FunctionArgument {
    name: String,
    arg_type: Type,
    pos_id: u32,
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
    module: Weak<Module>,
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
}

impl Function {
    pub fn from_ast(signature: &ast::FunctionSignature, module: Rc<Module>) -> Rc<Self> {
        let mut function = Function {
            args: vec![],
            return_type: Type::I64,
            entry_basic_block: Default::default(),
            module: Rc::downgrade(&module),
        };

        for (arg_id, arg_ast) in signature.args.iter().enumerate() {
            function.args.push(Rc::new(FunctionArgument {
                name: arg_ast.name.clone(),
                arg_type: Type::from_ast(&arg_ast.arg_type),
                pos_id: arg_id as u32,
            }));
        }

        Rc::new(function)
    }

    pub fn init_implementation(self: Rc<Self>, entry_basic_block_ast: &ast::BasicBlock) {
        self.entry_basic_block
            .set(BasicBlock::from_ast(
                &entry_basic_block_ast.statements,
                self.clone(),
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
}

impl<'ctx> Function {
    pub fn compile(&self, compiler: &FunctionCompiler<'ctx, '_>) {
        let entry_basic_block = self.entry_basic_block.get().unwrap();
        compiler.add_basic_block(entry_basic_block.clone());
    }
}

pub struct FunctionCompiler<'ctx, 'm> {
    pub module_compiler: &'m ModuleCompiler<'ctx>,
    pub ir: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionCompiler<'ctx, 'm> {
    type Target = ModuleCompiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.module_compiler
    }
}

impl<'ctx, 'm> FunctionCompiler<'ctx, 'm> {
    pub fn new(module_compiler: &'m ModuleCompiler<'ctx>, ir: FunctionValue<'ctx>) -> Self {
        let context = module_compiler.context();
        let builder = context.create_builder();
        FunctionCompiler {
            module_compiler,
            ir,
            builder,
        }
    }

    #[inline(always)]
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.ir.get_nth_param(arg.pos_id).unwrap()
    }

    fn add_basic_block(&self, basic_block: Rc<BasicBlock>) {
        let basic_block_ir = self.context().append_basic_block(self.ir, "");
        self.builder.position_at_end(basic_block_ir);

        let basic_block_compiler = BasicBlockCompiler::new(self);
        basic_block.compile(&basic_block_compiler);
    }
}
