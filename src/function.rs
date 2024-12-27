use crate::basic_block::{compile_basic_block, BasicBlock};
use crate::module::ModuleCompiler;
use crate::type_spec::TypeSpec;
use crate::value::Value;
use inkwell::basic_block::BasicBlock as BasicBlockIR;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};
use inkwell::values::{BasicValueEnum, FunctionValue};
use slotmap::DefaultKey;
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct FunctionArgument {
    pub name: String,
    pub arg_type: TypeSpec,
    pub pos_id: u32,
    pub ir_id: OnceCell<DefaultKey>,
}

pub struct Function {
    pub name: String,
    pub args: Vec<Rc<FunctionArgument>>,
    pub return_type: TypeSpec,
    pub entry: BasicBlock,
}

impl Function {
    pub fn compile_type<'ctx>(&self, builder: &ModuleCompiler<'ctx>) -> FunctionType<'ctx> {
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
    pub context: &'ctx Context,
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
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.ir.get_nth_param(arg.pos_id).unwrap()
    }

    pub fn add_basic_block(&self) -> BasicBlockIR<'ctx> {
        let basic_block = self.context().append_basic_block(self.ir, "entry");
        self.builder.position_at_end(basic_block);
        basic_block
    }
}

pub fn compile_function(function: Rc<Function>, module_compiler: &ModuleCompiler) {
    let builder = module_compiler.context().create_builder();
    let function_ir = module_compiler.add_function(function.as_ref());
    let function_compiler = FunctionCompiler {
        context: module_compiler.context(),
        module_compiler,
        ir: function_ir,
        builder,
    };

    for (pos_id, arg) in function.args.iter().enumerate() {
        let arg_ir = function_compiler.ir.get_nth_param(pos_id as u32).unwrap();
        let arg_value = match arg.arg_type {
            TypeSpec::I64 => Value::Integer(arg_ir.into_int_value()),
        };
        let arg_ir_id = function_compiler.store_value(arg_value);
        arg.ir_id.set(arg_ir_id).unwrap();
    }

    compile_basic_block(&function.entry, &function_compiler);
}
