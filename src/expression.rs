use crate::ast;
use crate::basic_block::BasicBlockCompiler;
use crate::function::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::ValueAssignment;
use crate::type_spec::{TypeHint, TypeSpec};
use inkwell::values::{BasicValue, BasicValueEnum};
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone)]
pub enum BinaryOperation {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    BitAnd,
    BitXor,
    BitOr,
    ShiftLeft,
    ShiftRight,
    Eq,
    Ne,
    Gt,
    Ge,
    Lt,
    Le,
    LogicalAnd,
    LogicalOr,
}

pub enum ExpressionNode {
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
    LoadIntegerConstant(i32),
    BinaryOperation(BinaryOperation, Box<ExpressionEdge>, Box<ExpressionEdge>),
}

impl ExpressionNode {
    pub fn from_ast(expression_ast: &ast::Expression, scope: &dyn LocalScope) -> Box<Self> {
        Box::new(match expression_ast {
            ast::Expression::Identifier(name) => {
                let resolved = scope.resolve(name).expect("not found");
                match resolved {
                    LocalScopeItem::Argument(arg) => ExpressionNode::LoadArgument(arg),
                    LocalScopeItem::Value(val) => ExpressionNode::LoadValue(val),
                }
            }
            ast::Expression::BinaryOperation(binary_operation_ast) => {
                let lhs = ExpressionEdge::from_ast(&binary_operation_ast.lhs, scope);
                let rhs = ExpressionEdge::from_ast(&binary_operation_ast.rhs, scope);
                ExpressionNode::BinaryOperation(binary_operation_ast.operation.clone(), lhs, rhs)
            }
            ast::Expression::Constant(constant) => match constant {
                ast::Constant::Void => todo!(),
                ast::Constant::True => todo!(),
                ast::Constant::False => todo!(),
                ast::Constant::Integer(value) => ExpressionNode::LoadIntegerConstant(*value),
                ast::Constant::Float(_) => todo!(),
                ast::Constant::String(_) => todo!(),
            },
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::UnaryOperation(_) => todo!(),
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
        })
    }
}

pub struct ExpressionEdge {
    node: Box<ExpressionNode>,
    type_spec: OnceCell<TypeSpec>,
}

impl ExpressionEdge {
    pub fn from_ast(expression_ast: &ast::Expression, scope: &dyn LocalScope) -> Box<Self> {
        let expression_edge = Box::new(ExpressionEdge {
            node: ExpressionNode::from_ast(expression_ast, scope),
            type_spec: Default::default(),
        });
        expression_edge
    }

    pub fn get_or_infer_type(&self, type_hint: &TypeHint) -> TypeSpec {
        if let Some(type_spec) = self.type_spec.get() {
            return type_spec.clone();
        }

        let type_spec = match self.node.as_ref() {
            ExpressionNode::LoadArgument(arg) => arg.arg_type(),
            ExpressionNode::LoadValue(val) => val.type_spec.clone(),
            ExpressionNode::LoadIntegerConstant(_) => TypeSpec::I64,
            ExpressionNode::BinaryOperation(op, lhs, rhs) => {
                dbg!();
                self.infer_binary_operation_type(type_hint, op.clone(), lhs, rhs)
            }
        };
        self.type_spec.set(type_spec.clone()).ok().unwrap();
        type_spec

        // match self.inner.as_ref() {
        //     Expression::Integer(_) => {}
        //     Expression::LoadArgument(_) => {}
        //     Expression::LoadValue(_) => {}
        // }
    }

    pub fn infer_binary_operation_type(
        &self,
        type_hint: &TypeHint,
        op: BinaryOperation,
        lhs: &Box<ExpressionEdge>,
        rhs: &Box<ExpressionEdge>,
    ) -> TypeSpec {
        todo!()
    }
}

#[repr(transparent)]
pub struct ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 'b> Deref for ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    type Target = BasicBlockCompiler<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.basic_block_compiler
    }
}

impl<'ctx, 'm, 'f, 'b> ExpressionCompiler<'ctx, 'm, 'f, 'b> {
    pub fn new(basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>) -> Self {
        ExpressionCompiler::<'ctx, 'm, 'f, 'b> {
            basic_block_compiler,
        }
    }

    pub fn compile_load_argument(&self, arg: Rc<FunctionArgument>) -> BasicValueEnum<'ctx> {
        self.load_argument(arg.as_ref())
    }

    pub fn compile_load_value(&self, val: Rc<ValueAssignment>) -> BasicValueEnum<'ctx> {
        let id = val.ir_id.get().unwrap();
        self.load_value(*id).unwrap()
    }

    pub fn compile_load_integer_constant(&self, value: i32) -> BasicValueEnum<'ctx> {
        let ctx = self.context();
        ctx.i64_type()
            .const_int(value as u64, true)
            .as_basic_value_enum()
    }

    pub fn compile_binary_operation(
        &self,
        op: BinaryOperation,
        lhs: &Box<ExpressionEdge>,
        rhs: &Box<ExpressionEdge>,
        type_spec: TypeSpec,
    ) -> BasicValueEnum<'ctx> {
        let lhs_ir = self.compile_expression(lhs);
        let rhs_ir = self.compile_expression(rhs);
        type_spec.compile_binary_operation(op, lhs_ir, rhs_ir, self)
    }

    pub fn compile_expression(&self, exp: &ExpressionEdge) -> BasicValueEnum<'ctx> {
        dbg!();
        let exp_type = exp.type_spec.get().unwrap().clone();
        match exp.node.as_ref() {
            ExpressionNode::LoadArgument(arg) => self.compile_load_argument(arg.clone()),
            ExpressionNode::LoadValue(val) => self.compile_load_value(val.clone()),
            ExpressionNode::LoadIntegerConstant(val) => self.compile_load_integer_constant(*val),
            ExpressionNode::BinaryOperation(op, lhs, rhs) => {
                self.compile_binary_operation(op.clone(), lhs, rhs, exp_type)
            }
        }
    }
}
