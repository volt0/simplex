use std::ops::Deref;
use std::rc::Rc;

use inkwell::values::{BasicValue, BasicValueEnum};

use crate::ast;
use crate::definitions::function::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::{StatementTranslator, ValueAssignment};
use crate::types::integer::IntegerExpressionTranslator;
use crate::types::{TypeHint, TypeSpec};

pub struct Expression {
    exp_type: TypeSpec,
    instruction: Instruction,
}

impl Expression {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        type_hint: &TypeHint,
        scope: &dyn LocalScope,
    ) -> Box<Self> {
        match type_hint {
            None => todo!(),
            Some(exp_type) => Box::new(Expression {
                exp_type: exp_type.clone(),
                instruction: Instruction::from_ast(exp_ast, type_hint, scope),
            }),
        }
    }
}

pub enum Instruction {
    LoadConstant(Constant),
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
    BinaryOperation(BinaryOperation, Box<Instruction>, Box<Instruction>),
    UnaryOperation(UnaryOperation, Box<Instruction>),
    // TypeAssertedSubtree(Box<Expression>),
    // Truncate(Box<IntegerExpression>),
}

impl Instruction {
    fn from_ast(
        exp_ast: &ast::Expression,
        type_hint: &TypeHint,
        scope: &dyn LocalScope,
    ) -> Instruction {
        match exp_ast {
            ast::Expression::Constant(const_ast) => {
                Instruction::LoadConstant(Constant::from_ast(const_ast))
            }
            ast::Expression::Identifier(name) => match scope.resolve(name).unwrap() {
                LocalScopeItem::Argument(arg) => Instruction::LoadArgument(arg),
                LocalScopeItem::Value(val) => Instruction::LoadValue(val),
            },
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::BinaryOperation(exp_ast) => {
                let lhs = Box::new(Self::from_ast(exp_ast.lhs.as_ref(), type_hint, scope));
                let rhs = Box::new(Self::from_ast(exp_ast.rhs.as_ref(), type_hint, scope));
                Instruction::BinaryOperation(exp_ast.operation.clone(), lhs, rhs)
            }
            ast::Expression::UnaryOperation(exp_ast) => {
                let arg = Box::new(Self::from_ast(exp_ast.arg.as_ref(), type_hint, scope));
                Instruction::UnaryOperation(exp_ast.operation.clone(), arg)
            }
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
        }
    }
}

pub enum Constant {
    Integer(i32),
}

impl Constant {
    pub fn from_ast(const_ast: &ast::Constant) -> Constant {
        match const_ast {
            ast::Constant::Void => todo!(),
            ast::Constant::True => todo!(),
            ast::Constant::False => todo!(),
            ast::Constant::Integer(val) => Constant::Integer(*val),
            ast::Constant::Float(_) => todo!(),
            ast::Constant::String(_) => todo!(),
        }
    }
}

#[derive(Clone, Debug)]
pub enum UnaryOperation {
    Plus,
    Minus,
    BitNot,
    LogicalNot,
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

pub struct ExpressionTranslator<'ctx, 'm, 'f, 'b> {
    parent: &'b StatementTranslator<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 'b> Deref for ExpressionTranslator<'ctx, 'm, 'f, 'b> {
    type Target = StatementTranslator<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx, 'm, 'f, 'b> ExpressionTranslator<'ctx, 'm, 'f, 'b> {
    pub fn new(parent: &'b StatementTranslator<'ctx, 'm, 'f>) -> Self {
        ExpressionTranslator::<'ctx, 'm, 'f, 'b> { parent }
    }

    pub fn translate_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_type = exp.exp_type.clone();
        match exp_type {
            TypeSpec::Void => todo!(),
            TypeSpec::Bool => todo!(),
            TypeSpec::Integer(integer_type) => {
                let translator = IntegerExpressionTranslator::new(self, integer_type);
                translator.translate_instruction(&exp.instruction)
            }
            TypeSpec::Float(_) => todo!(),
        }
    }

    pub fn translate_constant(&self, const_value: &Constant) -> BasicValueEnum<'ctx> {
        match const_value {
            Constant::Integer(value) => self
                .context
                .i64_type()
                .const_int(*value as u64, true)
                .as_basic_value_enum(),
        }
    }
}

#[cfg(test)]
mod tests {
    use inkwell::context::Context;
    use inkwell::execution_engine::JitFunction;

    use crate::ast;
    use crate::module::tests::compile_module_test;

    #[test]
    fn test_compile_expression() {
        let module_ast = ast::Module {
            definitions: vec![ast::Definition {
                name: "sum".to_string(),
                value: ast::DefinitionValue::Function(ast::Function {
                    signature: ast::FunctionSignature {
                        args: vec![
                            ast::FunctionArgument {
                                name: "x".to_string(),
                                arg_type: ast::Type::Integer(ast::IntegerType::I64),
                            },
                            ast::FunctionArgument {
                                name: "y".to_string(),
                                arg_type: ast::Type::Integer(ast::IntegerType::I64),
                            },
                            ast::FunctionArgument {
                                name: "z".to_string(),
                                arg_type: ast::Type::Integer(ast::IntegerType::I64),
                            },
                        ],
                        return_type: Some(ast::Type::Integer(ast::IntegerType::I64)),
                    },
                    body: ast::FunctionBody::BasicBlock(ast::BasicBlock {
                        statements: vec![ast::Statement::Return(Some(Box::new(
                            ast::Expression::BinaryOperation(ast::BinaryOperationExpr {
                                operation: ast::BinaryOperation::Add,
                                lhs: Box::new(ast::Expression::Constant(ast::Constant::Integer(8))),
                                rhs: Box::new(ast::Expression::Constant(ast::Constant::Integer(9))),
                            }),
                        )))],
                    }),
                }),
            }],
        };

        type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;
        let context = Context::create();
        let sum: JitFunction<SumFunc> = compile_module_test(module_ast, &context);
        let x = 1u64;
        let y = 2u64;
        let z = 3u64;
        unsafe {
            dbg!(sum.call(x, y, z));
        }
    }
}
