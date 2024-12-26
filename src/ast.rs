use std::rc::Rc;

// pub type Module = crate::module::Module;
// pub type Definition = crate::module::Definition;
// pub type DefinitionValue = crate::module::DefinitionValue;
// pub type Function = crate::function::Function;
// pub type FunctionArgument = crate::function::FunctionArgument;
// pub type FunctionSignature = crate::function::FunctionSignature;

pub enum Statement {
    Compound(CompoundStatement),
    Let(Rc<Variable>),
    Var(Rc<Variable>),
    If(ExpressionRef, CompoundStatement),
    While(ExpressionRef, CompoundStatement),
    For(Option<Rc<Variable>>, ExpressionRef, CompoundStatement),
    Break,
    Continue,
    Return(Option<ExpressionRef>),
    Expression(ExpressionRef),
}

pub struct CompoundStatement(pub Vec<Statement>);

pub struct Variable {
    pub name: Rc<str>,
    pub value_type: Option<TypeSpec>,
    pub init: Option<ExpressionRef>,
}

pub type ExpressionRef = Box<Expression>;

pub enum Expression {
    Constant(Constant),
    Identifier(Rc<str>),
    Conditional(ConditionalExpression),
    BinaryOperation(BinaryOperationExpr),
    UnaryOperation(UnaryOperationExpression),
    Cast(CastExpression),
    Call(CallExpression),
    ItemAccess(ItemAccessExpression),
    MemberAccess(MemberAccessExpression),
}

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

pub struct BinaryOperationExpr {
    pub operation: BinaryOperation,
    pub a: ExpressionRef,
    pub b: ExpressionRef,
}

pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
}

pub struct UnaryOperationExpression {
    pub operation: UnaryOperation,
    pub val: ExpressionRef,
}

pub struct ConditionalExpression(pub ExpressionRef, pub ExpressionRef, pub ExpressionRef);

pub struct CastExpression(pub ExpressionRef, pub TypeSpec);

pub struct CallExpression(pub ExpressionRef, pub Vec<ExpressionRef>);

pub struct ItemAccessExpression(pub ExpressionRef, pub ExpressionRef);

pub struct MemberAccessExpression(pub ExpressionRef, pub Rc<str>);

pub enum Constant {
    Void,
    True,
    False,
    Integer(i32),
    Float(f64),
    String(Rc<str>),
}

#[derive(Clone)]
pub enum TypeSpec {
    Identifier(Rc<str>),
    Void,
    Boolean,
    Integer(IntegerType),
    Float(FloatType),
}

#[derive(Clone)]
pub enum IntegerType {
    I8,
    I16,
    I32,
    I64,
    U8,
    U16,
    U32,
    U64,
}

#[derive(Clone)]
pub enum FloatType {
    F32,
    F64,
}
