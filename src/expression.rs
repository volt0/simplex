use crate::ast;
use crate::function::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::ValueAssignment;
use crate::types::{Type, TypeHint};
use std::rc::Rc;

#[derive(Debug)]
pub enum ExpressionNode {
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
    LoadIntegerConstant(i32),
    BinaryOperation(BinaryOperationExpression),
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
            ast::Expression::BinaryOperation(exp_ast) => ExpressionNode::BinaryOperation(
                BinaryOperationExpression::from_ast(exp_ast, scope, type_hint),
            ),
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

    fn infer_type(&self, type_hint: &TypeHint) -> Type {
        match self {
            ExpressionNode::LoadArgument(arg) => arg.arg_type(),
            ExpressionNode::LoadValue(val) => val.type_spec.clone(),
            ExpressionNode::LoadIntegerConstant(_) => Type::I64,
            ExpressionNode::BinaryOperation(op_exp) => op_exp.type_spec(type_hint),
        }
    }
}

#[derive(Debug)]
pub struct ExpressionEdge {
    node: Box<ExpressionNode>,
    exp_type: ExpressionType,
}

#[derive(Debug)]
enum ExpressionType {
    Use(Type),
}

impl ExpressionEdge {
    pub fn from_ast(
        expression_ast: &ast::Expression,
        scope: &dyn LocalScope,
        type_hint: &TypeHint,
    ) -> Box<Self> {
        let node = ExpressionNode::from_ast(expression_ast, scope, type_hint);
        let node_type = node.infer_type(type_hint);
        let expression_edge = Box::new(ExpressionEdge {
            node,
            exp_type: ExpressionType::Use(node_type),
        });
        expression_edge
    }

    pub fn exp_type(&self) -> Type {
        match &self.exp_type {
            ExpressionType::Use(type_spec) => type_spec.clone(),
        }
    }

    pub fn node(&self) -> &ExpressionNode {
        &self.node
    }
}

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
pub struct BinaryOperationExpression {
    pub op: BinaryOperation,
    pub lhs: Box<ExpressionEdge>,
    pub rhs: Box<ExpressionEdge>,
}

impl BinaryOperationExpression {
    fn from_ast(
        binary_operation_expr: &ast::BinaryOperationExpr,
        scope: &dyn LocalScope,
        type_hint: &TypeHint,
    ) -> Self {
        let lhs = ExpressionEdge::from_ast(&binary_operation_expr.lhs, scope, type_hint);
        let rhs = ExpressionEdge::from_ast(&binary_operation_expr.rhs, scope, type_hint);
        let op = binary_operation_expr.operation.clone();
        Self { op, lhs, rhs }
    }

    fn type_spec(&self, type_hint: &TypeHint) -> Type {
        match type_hint {
            TypeHint::Explicit(type_spec) => type_spec.clone(),
            TypeHint::Inferred => {
                let lhs_type = self.lhs.exp_type();
                let rhs_type = self.rhs.exp_type();
                assert_eq!(lhs_type, rhs_type);
                lhs_type
            }
        }
    }
}
