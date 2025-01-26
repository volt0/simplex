use crate::ast;
use crate::basic_block::BasicBlockCompiler;
use crate::function::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::ValueAssignment;
use crate::types::{Type, TypeHint};
use inkwell::values::{BasicValue, BasicValueEnum};
use std::ops::Deref;
use std::rc::Rc;

#[derive(Clone, Debug)]
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

#[derive(Debug)]
pub enum ExpressionNode {
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
    LoadIntegerConstant(i32),
    BinaryOperation(BinaryOperation, Box<ExpressionEdge>, Box<ExpressionEdge>),
}

impl ExpressionNode {
    pub fn from_ast(
        expression_ast: &ast::Expression,
        scope: &dyn LocalScope,
        type_hint: &TypeHint,
    ) -> Box<Self> {
        Box::new(match expression_ast {
            ast::Expression::Identifier(name) => {
                let resolved = scope.resolve(name).expect("not found");
                match resolved {
                    LocalScopeItem::Argument(arg) => ExpressionNode::LoadArgument(arg),
                    LocalScopeItem::Value(val) => ExpressionNode::LoadValue(val),
                }
            }
            ast::Expression::BinaryOperation(binary_operation_ast) => {
                let lhs = ExpressionEdge::from_ast(&binary_operation_ast.lhs, scope, type_hint);
                let rhs = ExpressionEdge::from_ast(&binary_operation_ast.rhs, scope, type_hint);
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

#[derive(Debug)]
pub struct ExpressionEdge {
    node: Box<ExpressionNode>,
    exp_type: ExpressionType,
}

#[derive(Debug)]
pub enum ExpressionType {
    Use(Type),
}

impl ExpressionEdge {
    pub fn from_ast(
        expression_ast: &ast::Expression,
        scope: &dyn LocalScope,
        type_hint: &TypeHint,
    ) -> Box<Self> {
        let node = ExpressionNode::from_ast(expression_ast, scope, type_hint);

        let type_spec = match node.as_ref() {
            ExpressionNode::LoadArgument(arg) => arg.arg_type(),
            ExpressionNode::LoadValue(val) => val.type_spec(),
            ExpressionNode::LoadIntegerConstant(_) => Type::I64,
            ExpressionNode::BinaryOperation(op, lhs, rhs) => {
                Self::infer_binary_operation_type(type_hint, op.clone(), lhs, rhs)
            }
        };

        let expression_edge = Box::new(ExpressionEdge {
            node,
            exp_type: ExpressionType::Use(type_spec),
        });
        expression_edge
    }

    pub fn type_spec(&self) -> Type {
        match &self.exp_type {
            ExpressionType::Use(type_spec) => type_spec.clone(),
        }
    }

    fn infer_binary_operation_type(
        type_hint: &TypeHint,
        op: BinaryOperation,
        lhs: &Box<ExpressionEdge>,
        rhs: &Box<ExpressionEdge>,
    ) -> Type {
        let lhs_type = lhs.type_spec();
        let rhs_type = rhs.type_spec();
        assert_eq!(lhs_type, rhs_type);
        lhs_type.clone()
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

    pub fn compile_expression(&self, exp: &ExpressionEdge) -> BasicValueEnum<'ctx> {
        let exp_type = exp.type_spec();
        match exp.node.as_ref() {
            ExpressionNode::LoadArgument(arg) => self.compile_load_argument(arg),
            ExpressionNode::LoadValue(val) => self.compile_load_value(val),
            ExpressionNode::LoadIntegerConstant(val) => self.compile_load_integer_constant(*val),
            ExpressionNode::BinaryOperation(op, lhs, rhs) => {
                self.compile_binary_operation(op.clone(), lhs, rhs, exp_type)
            }
        }
    }

    fn compile_load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.load_argument(arg)
    }

    fn compile_load_value(&self, val: &ValueAssignment) -> BasicValueEnum<'ctx> {
        let ir_id = val.ir_id();
        self.load_value(ir_id).unwrap()
    }

    fn compile_load_integer_constant(&self, value: i32) -> BasicValueEnum<'ctx> {
        let ctx = self.context();
        ctx.i64_type()
            .const_int(value as u64, true)
            .as_basic_value_enum()
    }

    fn compile_binary_operation(
        &self,
        op: BinaryOperation,
        lhs: &Box<ExpressionEdge>,
        rhs: &Box<ExpressionEdge>,
        type_spec: Type,
    ) -> BasicValueEnum<'ctx> {
        let lhs_ir = self.compile_expression(lhs);
        let rhs_ir = self.compile_expression(rhs);
        type_spec.compile_binary_operation(op, lhs_ir, rhs_ir, self)
    }
}
