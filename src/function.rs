use std::ops::Deref;
use inkwell::context::Context as BackendContext;
use inkwell::values::FunctionValue;

use crate::module::Module;

pub struct Function<'ctx> {
    ir: FunctionValue<'ctx>,
}

impl<'ctx> Deref for Function<'ctx> {
    type Target = FunctionValue<'ctx>;

    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<'ctx> Function<'ctx> {
    pub fn new(name: &str, module: &Module<'ctx>, ctx: &'ctx BackendContext) -> Self {
        let type_ir = ctx.i32_type().fn_type(&[ctx.i32_type().into()], false);

        Function {
            ir: module.add_function(name, type_ir, None),
        }
    }

    pub fn compile(
        &self,
        body: Vec<crate::ast::Statement>,
        module: &Module<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        dbg!(self.ir.get_name());
    }
}
