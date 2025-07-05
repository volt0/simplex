mod integer;

use crate::ast;
use crate::basic_block::BasicBlockCompiler;
use crate::function::FunctionArgument;
use crate::scope::{LocalScope, LocalScopeItem};
use crate::statement::ValueAssignment;
use crate::types::{IntegerType, TypeHint};

use inkwell::values::BasicValueEnum;
use integer::IntegerExpression;
use std::ops::Deref;
use std::rc::Rc;

#[derive(Debug)]
pub enum Expression {
    Integer(Box<IntegerExpression>),
    LoadArgument(Rc<FunctionArgument>),
    LoadValue(Rc<ValueAssignment>),
}

impl Expression {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        scope: &dyn LocalScope,
        type_hint: &TypeHint,
    ) -> Box<Self> {
        Box::new(match exp_ast {
            ast::Expression::Identifier(name) => {
                let resolved = scope.resolve(name).expect("not found");
                match resolved {
                    LocalScopeItem::Argument(arg) => Expression::LoadArgument(arg),
                    LocalScopeItem::Value(val) => Expression::LoadValue(val),
                }
            }
            exp_ast => match type_hint {
                TypeHint::Integer(_) => {
                    Expression::Integer(IntegerExpression::from_ast(exp_ast, scope, type_hint))
                }
                TypeHint::Inferred => todo!(),
            },
        })

        // Box::new()
    }
}
impl<'ctx> Expression {
    pub fn compile(&self, ctx: &ExpressionContext<'ctx, '_, '_, '_>) -> BasicValueEnum<'ctx> {
        match self {
            Expression::Integer(int_exp) => {
                let int_type = IntegerType::from_ast(&ast::IntegerType::I64);
                let result = int_exp.compile(&int_type, &ctx);
                result.into()
            }
            Expression::LoadArgument(arg) => self.load_argument(arg, ctx),
            Expression::LoadValue(val) => self.load_value_assignment(val, ctx),
        }
    }

    fn load_value_assignment(
        &self,
        val: &ValueAssignment,
        ctx: &ExpressionContext<'ctx, '_, '_, '_>,
    ) -> BasicValueEnum<'ctx> {
        ctx.load_value_assignment(val)
    }

    fn load_argument(
        &self,
        arg: &FunctionArgument,
        ctx: &ExpressionContext<'ctx, '_, '_, '_>,
    ) -> BasicValueEnum<'ctx> {
        ctx.load_argument(arg)
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

#[repr(transparent)]
pub struct ExpressionContext<'ctx, 'm, 'f, 'b> {
    basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>,
}

impl<'ctx, 'm, 'f, 'b> Deref for ExpressionContext<'ctx, 'm, 'f, 'b> {
    type Target = BasicBlockCompiler<'ctx, 'm, 'f>;

    fn deref(&self) -> &Self::Target {
        self.basic_block_compiler
    }
}

impl<'ctx, 'm, 'f, 'b> ExpressionContext<'ctx, 'm, 'f, 'b> {
    pub fn new(basic_block_compiler: &'b BasicBlockCompiler<'ctx, 'm, 'f>) -> Self {
        ExpressionContext::<'ctx, 'm, 'f, 'b> {
            basic_block_compiler,
        }
    }

    pub fn load_value_assignment(&self, val: &ValueAssignment) -> BasicValueEnum<'ctx> {
        let ir_id = val.ir_id.get().unwrap().clone();
        self.load_value(ir_id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::module::tests::compile_module_test;
    use inkwell::context::Context;
    use inkwell::execution_engine::JitFunction;

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
                        statements: vec![
                            ast::Statement::Let(ast::Variable {
                                name: "a".to_string(),
                                value_type: Some(ast::Type::Integer(ast::IntegerType::I64)),
                                init_expression: Some(Box::new(ast::Expression::Constant(
                                    ast::Constant::Integer(10),
                                ))),
                            }),
                            ast::Statement::Return(Some(Box::new(
                                ast::Expression::BinaryOperation(ast::BinaryOperationExpr {
                                    operation: ast::BinaryOperation::Add,
                                    lhs: Box::new(ast::Expression::Identifier("x".to_string())),
                                    rhs: Box::new(ast::Expression::BinaryOperation(
                                        ast::BinaryOperationExpr {
                                            operation: ast::BinaryOperation::Add,
                                            lhs: Box::new(ast::Expression::Identifier(
                                                "y".to_string(),
                                            )),
                                            rhs: Box::new(ast::Expression::BinaryOperation(
                                                ast::BinaryOperationExpr {
                                                    operation: ast::BinaryOperation::Add,
                                                    lhs: Box::new(ast::Expression::Identifier(
                                                        "z".to_string(),
                                                    )),
                                                    rhs: Box::new(ast::Expression::Identifier(
                                                        "a".to_string(),
                                                    )),
                                                },
                                            )),
                                        },
                                    )),
                                }),
                            ))),
                        ],
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
