use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::FunctionValue;

use crate::scope::Scope;
use crate::statements::local_scope::LocalScope;
use crate::statements::Statement;

#[repr(transparent)]
pub struct CompoundStatement(pub Vec<Statement>);

impl CompoundStatement {
    pub fn compile<'ctx>(
        &self,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        function_ir: FunctionValue<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        let mut scope = LocalScope {
            index: Default::default(),
            parent: scope,
        };

        let entry_block = ctx.append_basic_block(function_ir, "");
        builder.position_at_end(entry_block);

        for statement in self.0.iter() {
            statement.compile(&mut scope, builder, function_ir, ctx);
        }
    }
}
