use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::ast;
use crate::module::Module;
use crate::scope::{Identifier, LocalScope, Scope};

pub struct Function<'ctx> {
    ir: FunctionValue<'ctx>,
    signature: ast::FunctionSignature,
}

// impl<'ctx> Deref for Function<'ctx> {
//     type Target = FunctionValue<'ctx>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.ir
//     }
// }

impl<'ctx> Function<'ctx> {
    pub fn new(ir: FunctionValue<'ctx>, signature: ast::FunctionSignature) -> Self {
        Function { ir, signature }
    }
}

impl<'ctx> Function<'ctx> {
    pub fn compile(&self, payload: ast::CompoundStatement, module: &Module<'ctx>) {
        let ctx = module.context();

        let mut scope = LocalScope::new(module);
        for (arg_id, arg) in self.signature.args.iter().enumerate() {
            let arg_ir = self.ir.get_nth_param(arg_id as u32).unwrap();
            scope.insert(
                arg.name.clone(),
                Identifier::new_argument(arg.clone(), arg_ir),
            );
        }

        let entry_block = ctx.append_basic_block(self.ir, "entry");
        let builder = ctx.create_builder();
        builder.position_at_end(entry_block);

        self.add_compound_statement(payload, &scope, &builder, module, ctx);
    }

    fn add_compound_statement(
        &self,
        stmt: ast::CompoundStatement,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        for statement in stmt.0 {
            match statement {
                ast::Statement::Compound(inner) => {
                    self.add_compound_statement(inner, scope, builder, module, ctx)
                }
                ast::Statement::Let(_) => todo!(),
                ast::Statement::Var(_) => todo!(),
                ast::Statement::If(_, _) => todo!(),
                ast::Statement::While(_, _) => todo!(),
                ast::Statement::For(_, _, _) => todo!(),
                ast::Statement::Break => todo!(),
                ast::Statement::Continue => todo!(),
                ast::Statement::Return(expr) => {
                    let res = expr
                        .and_then(|expr| self.add_expression(expr, scope, builder, module, ctx));

                    builder
                        .build_return(res.as_ref().map(|val| val as _))
                        .unwrap();
                }
                ast::Statement::Expression(_) => todo!(),
            }
        }
    }

    fn add_expression(
        &self,
        expr: ast::ExpressionRef,
        scope: &dyn Scope<'ctx>,
        builder: &Builder<'ctx>,
        module: &Module<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Option<BasicValueEnum> {
        match expr.as_ref() {
            ast::Expression::Constant(inner) => match inner {
                ast::Constant::Void => None,
                ast::Constant::True => Some(ctx.bool_type().const_int(1, false).into()),
                ast::Constant::False => Some(ctx.bool_type().const_int(0, false).into()),
                ast::Constant::Integer(val) => {
                    Some(ctx.i32_type().const_int(*val as u64, true).into())
                }
                ast::Constant::Float(val) => Some(ctx.f64_type().const_float(*val).into()),
                ast::Constant::String(_) => todo!(),
            },
            ast::Expression::Identifier(name) => match scope.lookup(name.as_ref()).unwrap() {
                Identifier::Value(value) => Some(value.ir),
            },
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::BinaryOperation(_) => todo!(),
            ast::Expression::UnaryOperation(_) => todo!(),
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
        }
    }
}
