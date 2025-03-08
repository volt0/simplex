#![allow(unused)]

pub struct Module {
    pub definitions: Vec<Definition>,
}

pub struct Definition {
    pub name: String,
    pub value: DefinitionValue,
}

pub enum DefinitionValue {
    Function(Function),
}

pub struct FunctionArgument {
    pub name: String,
    pub arg_type: TypeSpec,
}

pub struct FunctionSignature {
    pub args: Vec<FunctionArgument>,
    pub return_type: Option<TypeSpec>,
}

pub struct Function {
    pub signature: FunctionSignature,
    pub payload: Option<BasicBlock>,
}

pub struct Variable {
    pub name: String,
    pub value_type: Option<TypeSpec>,
    pub init_expression: Option<Box<Expression>>,
}

pub struct BasicBlock {
    pub statements: Vec<Statement>,
}

pub enum Statement {
    BasicBlock(BasicBlock),
    Let(Variable),
    Var(Variable),
    If(Box<Expression>, BasicBlock),
    While(Box<Expression>, BasicBlock),
    For(Option<Variable>, Box<Expression>, BasicBlock),
    Break,
    Continue,
    Return(Option<Box<Expression>>),
    Expression(Box<Expression>),
}

pub enum Expression {
    Constant(Constant),
    Identifier(String),
    Conditional(ConditionalExpression),
    BinaryOperation(BinaryOperationExpr),
    UnaryOperation(UnaryOperationExpression),
    Cast(CastExpression),
    Call(CallExpression),
    ItemAccess(ItemAccessExpression),
    MemberAccess(MemberAccessExpression),
}

pub type BinaryOperation = crate::expression::BinaryOperation;

pub struct BinaryOperationExpr {
    pub operation: BinaryOperation,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
}

pub struct UnaryOperationExpression {
    pub operation: UnaryOperation,
    pub val: Box<Expression>,
}

pub struct ConditionalExpression(
    pub Box<Expression>,
    pub Box<Expression>,
    pub Box<Expression>,
);

pub struct CastExpression(pub Box<Expression>, pub TypeSpec);

pub struct CallExpression(pub Box<Expression>, pub Vec<Box<Expression>>);

pub struct ItemAccessExpression(pub Box<Expression>, pub Box<Expression>);

pub struct MemberAccessExpression(pub Box<Expression>, pub String);

pub enum Constant {
    Void,
    True,
    False,
    Integer(i32),
    Float(f64),
    String(String),
}

#[derive(Clone)]
pub enum TypeSpec {
    Identifier(String),
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
