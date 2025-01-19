use crate::ast;
use crate::basic_block::BasicBlock;
use crate::module::{Module, ModuleCompiler};
use crate::scope::{LocalScope, LocalScopeItem};
use crate::type_spec::TypeSpec;
use inkwell::basic_block::BasicBlock as BasicBlockIR;
use inkwell::builder::Builder;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::{Rc, Weak};

pub struct FunctionArgument {
    name: String,
    arg_type: TypeSpec,
    pos_id: u32,
}

impl FunctionArgument {
    pub fn arg_type(&self) -> TypeSpec {
        self.arg_type.clone()
    }
}

pub struct Function {
    args: Vec<Rc<FunctionArgument>>,
    return_type: TypeSpec,
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
            return_type: TypeSpec::I64,
            entry_basic_block: Default::default(),
            module: Rc::downgrade(&module),
        };

        for (arg_id, arg_ast) in signature.args.iter().enumerate() {
            function.args.push(Rc::new(FunctionArgument {
                name: arg_ast.name.clone(),
                arg_type: TypeSpec::from_ast(&arg_ast.arg_type),
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
}

impl<'ctx> Function {
    pub fn compile(&self, compiler: &FunctionCompiler<'ctx, '_>) {
        let basic_block = compiler.add_basic_block();
        compiler.builder.position_at_end(basic_block);

        let entry_basic_block = self.entry_basic_block.get().unwrap();
        entry_basic_block.compile(&compiler);
    }

    pub fn compile_type(&self, builder: &ModuleCompiler<'ctx>) -> FunctionType<'ctx> {
        let return_type_ir = self.return_type.clone().into_ir(&builder.context());

        let arg_type_irs: Vec<BasicMetadataTypeEnum> = self
            .args
            .iter()
            .map(|arg| arg.arg_type.clone().into_ir(&builder.context()).into())
            .collect();

        return_type_ir.fn_type(&arg_type_irs, false)
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

    pub fn add_basic_block(&self) -> BasicBlockIR<'ctx> {
        self.context().append_basic_block(self.ir, "")
    }
}
