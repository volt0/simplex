use crate::basic_block::{BasicBlock, BasicBlockCompiler};
use crate::module::ModuleCompiler;
use crate::type_spec::TypeSpec;
use crate::value::Value;
use inkwell::context::Context;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};
use inkwell::values::FunctionValue;
use slotmap::DefaultKey;
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct FunctionArgument {
    pub name: String,
    pub arg_type: TypeSpec,
    pub ir_id: OnceCell<DefaultKey>,
}

pub struct Function {
    pub name: String,
    pub args: Vec<Rc<FunctionArgument>>,
    pub return_type: TypeSpec,
    pub entry: BasicBlock,
}

impl Function {
    pub fn compile(&self, compiler: &FunctionCompiler) {
        for (pos_id, arg) in self.args.iter().enumerate() {
            let arg_ir = compiler.ir.get_nth_param(pos_id as u32).unwrap();
            let arg_value = match arg.arg_type {
                TypeSpec::I64 => Value::Integer(arg_ir.into_int_value()),
            };
            let arg_ir_id = compiler.store_value(arg_value);
            arg.ir_id.set(arg_ir_id).unwrap();
        }
        let basic_block_compiler = compiler.add_basic_block();
        self.entry.compile(&basic_block_compiler)
    }

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
}

impl<'ctx, 'm> Deref for FunctionCompiler<'ctx, 'm> {
    type Target = ModuleCompiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.module_compiler
    }
}

impl<'ctx, 'm> FunctionCompiler<'ctx, 'm> {
    pub fn add_basic_block(&self) -> BasicBlockCompiler<'ctx, 'm, '_> {
        let builder = self.context.create_builder();
        let basic_block = self.context.append_basic_block(self.ir, "entry");
        builder.position_at_end(basic_block);

        BasicBlockCompiler {
            context: self.context,
            function_compiler: self,
            builder,
            basic_block,
        }
    }
}
