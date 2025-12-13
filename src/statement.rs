use crate::ast;
use crate::basic_block::BasicBlockVisitor;
use crate::expression::{Expression, ExpressionTranslator};
use crate::function::FunctionCompiler;
use crate::integer::Integer;
use crate::scope::LocalScope;
use crate::types::Type;
use inkwell::values::BasicValueEnum;
use slotmap::DefaultKey;
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

pub enum Statement {
    ValueAssignment(Rc<ValueAssignment>),
    Return(Box<Expression>),
}

impl Statement {
    pub fn from_ast(statement_ast: ast::Statement, scope: &dyn LocalScope) -> Self {
        match statement_ast {
            ast::Statement::Let(var_ast) => Self::from_ast_let(&var_ast, scope),
            ast::Statement::Return(exp) => match exp {
                None => todo!(),
                Some(exp) => Self::from_ast_return(&exp, scope),
            },
            ast::Statement::BasicBlock(_) => todo!(),
            ast::Statement::Var(_) => todo!(),
            ast::Statement::If(_, _) => todo!(),
            ast::Statement::While(_, _) => todo!(),
            ast::Statement::For(_, _, _) => todo!(),
            ast::Statement::Break => todo!(),
            ast::Statement::Continue => todo!(),
            ast::Statement::Expression(_) => todo!(),
        }
    }

    pub fn from_ast_let(var_ast: &ast::Variable, scope: &dyn LocalScope) -> Self {
        let value_type_ast = var_ast.value_type.as_ref();
        todo!()
        // let type_hint = value_type_ast.map(|type_ast| TypeSpec::from_ast(type_ast));
        // let exp_ast = var_ast.init_expression.as_ref().unwrap();
        // let exp = Expression::from_ast(exp_ast, scope, type_hint);
        // Statement::ValueAssignment(ValueAssignment::new(var_ast.name.clone(), exp))
    }

    pub fn from_ast_return(exp_ast: &ast::Expression, scope: &dyn LocalScope) -> Self {
        // let function = scope.current_function();
        // let type_hint = match function.return_type() {
        //     TypeSpec::Void => todo!(),
        //     TypeSpec::Primitive(prim_type) => match prim_type {
        //         PrimitiveType::Void => todo!(),
        //         PrimitiveType::Bool => todo!(),
        //         PrimitiveType::Integer(int_type) => {
        //             TypeSpec::Primitive(PrimitiveType::Integer(int_type))
        //         }
        //         PrimitiveType::Float(_) => todo!(),
        //     },
        //     TypeSpec::Function(_) => todo!(),
        // };
        let type_hint = Rc::new(Integer { is_signed: false }) as Rc<dyn Type>;

        Statement::Return(Expression::from_ast(exp_ast, &type_hint, scope))
    }
}

pub struct ValueAssignment {
    pub name: String,
    pub exp: Box<Expression>,
    pub ir_id: OnceCell<DefaultKey>,
}

impl ValueAssignment {
    pub fn new(name: String, exp: Box<Expression>) -> Rc<Self> {
        Rc::new(ValueAssignment {
            name,
            exp,
            ir_id: Default::default(),
        })
    }
}

#[repr(transparent)]
pub struct StatementCompiler<'ctx, 'm, 'f> {
    parent: &'f FunctionCompiler<'ctx, 'm>,
}

impl<'ctx, 'm, 'f> Deref for StatementCompiler<'ctx, 'm, 'f> {
    type Target = FunctionCompiler<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f> BasicBlockVisitor for StatementCompiler<'ctx, 'm, 'f> {
    fn visit_statement(&self, stmt: &Statement) {
        self.compile_statement(stmt);
    }
}

impl<'ctx, 'm, 'f> StatementCompiler<'ctx, 'm, 'f> {
    pub fn new(parent: &'f FunctionCompiler<'ctx, 'm>) -> Self {
        Self { parent }
    }

    pub fn compile_statement(&self, stmt: &Statement) {
        match stmt {
            Statement::ValueAssignment(var) => {
                self.add_statement_let(var);
            }
            Statement::Return(exp) => {
                self.add_statement_return(exp);
            }
        }
    }

    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_compiler = ExpressionTranslator::new(self);
        exp_compiler.translate_expression(exp)
    }

    fn add_statement_let(&self, val: &ValueAssignment) {
        let value = self.compile_expression(val.exp.as_ref());
        let value_id = self.store_value(value);
        val.ir_id.set(value_id).unwrap();
    }

    fn add_statement_return(&self, exp: &Expression) {
        let result = self.compile_expression(exp);
        self.builder.build_return(Some(&result)).unwrap();
    }
}
