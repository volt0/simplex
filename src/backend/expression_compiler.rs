use super::basic_block_compiler::BasicBlockCompiler;
use super::codegen::integer_ops::IntegerExpressionCodegen;
use crate::expression::Expression;
use crate::statement::ValueAssignment;
use crate::types::{PrimitiveType, Type};
use inkwell::values::BasicValueEnum;
use std::ops::Deref;

pub trait ExpressionCodegen<'ctx> {
    fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx>;
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

    pub fn compile_expression(&self, exp: &Expression) -> BasicValueEnum<'ctx> {
        let exp_type = exp.exp_type.clone();
        let codegen = match exp_type {
            Type::Primitive(exp_type) => match exp_type {
                PrimitiveType::Void => todo!(),
                PrimitiveType::Bool => todo!(),
                PrimitiveType::Integer(int_type) => IntegerExpressionCodegen::new(self, int_type),
                PrimitiveType::Float(_) => todo!(),
            },
            Type::Function(_) => todo!(),
        };
        codegen.compile_expression(exp)
    }

    pub fn load_value_assignment(&self, val: &ValueAssignment) -> BasicValueEnum<'ctx> {
        let ir_id = val.ir_id.get().unwrap().clone();
        self.load_value(ir_id).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use crate::ast;
    use crate::backend::module_compiler::tests::compile_module_test;
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
