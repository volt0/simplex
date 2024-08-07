use std::ops::Deref;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::{BasicValueEnum, FunctionValue};

use crate::ast;
use crate::module::Module;
use crate::scope::{Identifier, Scope};
use crate::types::Type;

pub struct Function<'ctx> {
    ir: FunctionValue<'ctx>,
    signature: ast::FunctionSignature,
}

impl<'ctx> Deref for Function<'ctx> {
    type Target = FunctionValue<'ctx>;

    fn deref(&self) -> &Self::Target {
        &self.ir
    }
}

impl<'ctx> Function<'ctx> {
    pub fn new(
        name: &str,
        signature: ast::FunctionSignature,
        module: &Module<'ctx>,
        ctx: &'ctx BackendContext,
    ) -> Self {
        let type_ir = {
            let signature = signature.clone();
            let args_ast = signature.args;
            let mut arg_types = vec![];
            for arg in args_ast {
                let arg_type_ast = arg.type_spec;
                let arg_type = Type::compile(arg_type_ast, ctx);
                let arg_type_ir: BasicTypeEnum = arg_type.try_into().unwrap();
                arg_types.push(arg_type_ir.into());
            }

            let return_type_ast = signature.return_type.unwrap_or(ast::TypeSpec::Void);
            let return_type = Type::compile(return_type_ast, ctx);

            let is_var_args = false;
            match return_type {
                Type::Void(_) => ctx.void_type().fn_type(&arg_types, is_var_args),
                return_type => {
                    let return_type_ir: BasicTypeEnum = return_type.try_into().unwrap();
                    return_type_ir.fn_type(&arg_types, is_var_args)
                }
            }
        };

        let function_ir = module.add_function(name, type_ir, None);
        Function {
            ir: function_ir,
            signature,
        }
    }
}

impl<'ctx, 's> Function<'ctx> {
    pub fn compile(
        &self,
        payload: ast::CompoundStatement,
        scope: &Scope<'ctx, 's>,
        module: &Module<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        let mut scope = Scope::new(scope);
        for (arg_id, arg) in self.signature.args.iter().enumerate() {
            let arg_ir = self.get_nth_param(arg_id as u32).unwrap();
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
        scope: &Scope<'ctx, 's>,
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
        scope: &Scope<'ctx, 's>,
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
            ast::Expression::Identifier(name) => match scope.lookup(name.clone()).unwrap() {
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
