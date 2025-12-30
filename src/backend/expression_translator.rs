use std::ops::Deref;

use inkwell::values::{BasicValue, BasicValueEnum};

use super::statement_translator::StatementTranslator;
use super::types_impl::IntegerExpressionTranslator;
use crate::constant::Constant;
use crate::expression::Expression;
use crate::types::TypeSpec;

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
        let exp_type = exp.get_type();
        match exp_type {
            TypeSpec::Void => todo!(),
            TypeSpec::Bool => todo!(),
            TypeSpec::Integer(integer_type) => {
                let translator = IntegerExpressionTranslator::new(self, integer_type);
                translator.translate_instruction(&exp.get_instruction())
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

    use super::super::module_translator::tests::compile_module_test;
    use crate::ast;

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
