use crate::ast;
use crate::scope::LocalScope;
use crate::statement::StatementTranslator;
use crate::types::{Type, TypeHint};
use inkwell::values::{BasicValue, BasicValueEnum};
use std::ops::Deref;
use std::rc::Rc;

pub struct Expression {
    exp_type: Rc<dyn Type>,
    instruction: Instruction,
}

impl Expression {
    pub fn from_ast(
        exp_ast: &ast::Expression,
        type_hint: &TypeHint,
        scope: &dyn LocalScope,
    ) -> Box<Self> {
        Box::new(Expression {
            exp_type: type_hint.clone(),
            instruction: Self::from_ast_instruction(exp_ast, type_hint, scope),
        })
    }

    fn from_ast_instruction(
        exp_ast: &ast::Expression,
        type_hint: &TypeHint,
        scope: &dyn LocalScope,
    ) -> Instruction {
        match exp_ast {
            ast::Expression::Constant(const_ast) => Self::from_ast_constant(const_ast),
            ast::Expression::Identifier(_) => todo!(),
            ast::Expression::Conditional(_) => todo!(),
            ast::Expression::BinaryOperation(exp_ast) => {
                let lhs = Self::from_ast_instruction(exp_ast.lhs.as_ref(), type_hint, scope);
                let rhs = Self::from_ast_instruction(exp_ast.rhs.as_ref(), type_hint, scope);
                Instruction::Binary(exp_ast.operation.clone(), Box::new(lhs), Box::new(rhs))
            }
            ast::Expression::UnaryOperation(_) => todo!(),
            ast::Expression::Cast(_) => todo!(),
            ast::Expression::Call(_) => todo!(),
            ast::Expression::ItemAccess(_) => todo!(),
            ast::Expression::MemberAccess(_) => todo!(),
        }
    }

    fn from_ast_constant(const_ast: &ast::Constant) -> Instruction {
        Instruction::LoadConstant(Constant::from_ast(const_ast))
    }
}

pub enum Instruction {
    LoadConstant(Constant),
    Binary(BinaryOperation, Box<Instruction>, Box<Instruction>),
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

pub trait TypedExpressionTranslator {
    fn translate_binary_operation<'ctx, 'm, 'f, 'b>(
        &self,
        op: &BinaryOperation,
        lhs: &Instruction,
        rhs: &Instruction,
        parent: &ExpressionTranslator<'ctx, 'm, 'f, 'b>,
    ) -> BasicValueEnum<'ctx>;
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
        let exp_type = exp.exp_type.as_ref();
        let exp_translator = exp_type.create_expression_translator();
        self.translate_instruction(&exp.instruction, exp_translator.as_ref())
    }

    pub fn translate_instruction(
        &self,
        instruction: &Instruction,
        exp_translator: &dyn TypedExpressionTranslator,
    ) -> BasicValueEnum<'ctx> {
        match instruction {
            Instruction::LoadConstant(const_value) => self.translate_constant(const_value),
            Instruction::Binary(op, lhs, rhs) => {
                exp_translator.translate_binary_operation(op, lhs, rhs, self)
            }
        }
    }

    fn translate_constant(&self, const_value: &Constant) -> BasicValueEnum<'ctx> {
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
