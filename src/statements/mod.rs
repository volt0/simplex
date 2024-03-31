use std::rc::Rc;

use inkwell::builder::Builder;
use inkwell::context::Context as BackendContext;
use inkwell::values::FunctionValue;

pub use compound::CompoundStatement;
pub use local_scope::LocalScope;
pub use variable::Variable;

use crate::expressions::ExpressionRef;
use crate::types::TypeSpec;

mod compound;
mod local_scope;
mod variable;

pub enum Statement {
    Compound(CompoundStatement),
    Let(Rc<Variable>),
    Return(ExpressionRef),
}

impl Statement {
    #[inline(always)]
    pub fn new_compound(inner: CompoundStatement) -> Self {
        Statement::Compound(inner)
    }

    #[inline(always)]
    pub fn new_assign() -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn new_let(name: Rc<str>, value_type: Option<TypeSpec>, value_init: ExpressionRef) -> Self {
        Statement::Let(Variable::new(name, value_type, value_init))
    }

    #[inline(always)]
    pub fn new_var() -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn new_if() -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn new_while() -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn new_for() -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn new_break() -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn new_continue() -> Self {
        todo!()
    }

    #[inline(always)]
    pub fn new_return(expression: Option<ExpressionRef>) -> Self {
        match expression {
            None => todo!(),
            Some(expression) => Statement::Return(expression),
        }
    }
}

impl Statement {
    pub fn compile<'ctx, 'a>(
        &self,
        scope: &mut LocalScope<'ctx, 'a>,
        builder: &Builder<'ctx>,
        function_ir: FunctionValue<'ctx>,
        ctx: &'ctx BackendContext,
    ) {
        match self {
            Statement::Compound(inner) => inner.compile(scope, builder, function_ir, ctx),
            Statement::Let(variable) => {
                let value = variable.compile(scope, builder, ctx);
                scope.index.insert(variable.name.clone(), value);
            }
            Statement::Return(expression) => {
                let expression = expression.as_ref();
                let return_value = expression.compile(scope, builder, ctx);
                builder.build_return(Some(&return_value.ir)).unwrap();
            }
        }

        // let w = builder
        //     .build_call(fn2, &[], "w")
        //     .unwrap()
        //     .try_as_basic_value()
        //     .unwrap_left()
        //     .into_int_value();
    }
}
