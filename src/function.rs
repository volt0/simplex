use crate::ast;
use crate::basic_block::BasicBlock;
use crate::module::ModuleCompiler;
use crate::type_spec::TypeSpec;
use inkwell::basic_block::BasicBlock as BasicBlockIR;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, FunctionType};
use inkwell::values::{BasicValueEnum, FunctionValue};
use std::cell::OnceCell;
use std::ops::Deref;
use std::rc::Rc;

pub struct FunctionArgument {
    pub name: String,
    pub arg_type: TypeSpec,
    pub pos_id: u32,
}

pub struct Function {
    pub args: Vec<Rc<FunctionArgument>>,
    pub return_type: TypeSpec,
    pub entry_basic_block: OnceCell<BasicBlock>,
}

impl Function {
    pub fn from_ast(signature: &ast::FunctionSignature) -> Rc<Self> {
        let mut function = Function {
            args: vec![],
            return_type: TypeSpec::I64,
            entry_basic_block: Default::default(),
        };

        let mut arg_id = 0;
        for arg_ast in &signature.args {
            function.args.push(Rc::new(FunctionArgument {
                name: arg_ast.name.clone(),
                arg_type: TypeSpec::from_ast(&arg_ast.arg_type),
                pos_id: arg_id,
            }));
            arg_id += 1;
        }

        Rc::new(function)
    }

    pub fn init_implementation(&self, entry_basic_block_ast: &ast::BasicBlock) {
        use crate::expression::{BinaryOperation, Expression, IntegerExpression};
        use crate::statement::{Statement, Value};

        let x = Rc::new(FunctionArgument {
            name: "x".to_string(),
            arg_type: TypeSpec::I64,
            pos_id: 0,
        });
        let y = Rc::new(FunctionArgument {
            name: "y".to_string(),
            arg_type: TypeSpec::I64,
            pos_id: 1,
        });
        let z = Rc::new(FunctionArgument {
            name: "z".to_string(),
            arg_type: TypeSpec::I64,
            pos_id: 2,
        });
        let a = Rc::new(Value {
            type_spec: TypeSpec::I64,
            assigned_exp: Box::new(
                IntegerExpression::BinaryOperation(
                    BinaryOperation::Add,
                    Box::new(IntegerExpression::Force(Box::new(
                        Expression::LoadArgument(x.clone()),
                    ))),
                    Box::new(IntegerExpression::Force(Box::new(
                        Expression::LoadArgument(y.clone()),
                    ))),
                )
                .into(),
            ),
            ir_id: Default::default(),
        });

        let entry_basic_block = BasicBlock {
            statements: vec![
                Statement::Let(a.clone()),
                Statement::Return(Box::new(Expression::Integer(
                    IntegerExpression::BinaryOperation(
                        BinaryOperation::Add,
                        Box::new(IntegerExpression::Force(Box::new(Expression::LoadValue(a)))),
                        Box::new(IntegerExpression::Force(Box::new(
                            Expression::LoadArgument(z),
                        ))),
                    ),
                ))),
            ],
        };

        // let entry_basic_block = BasicBlock::from_ast(&entry_basic_block_ast.statements);
        self.entry_basic_block.set(entry_basic_block).ok().unwrap();
    }
}

pub struct FunctionCompiler<'ctx, 'm> {
    pub context: &'ctx Context,
    pub module_compiler: &'m ModuleCompiler<'ctx>,
    pub ir: FunctionValue<'ctx>,
    builder: Builder<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionCompiler<'ctx, 'm> {
    type Target = ModuleCompiler<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.module_compiler
    }
}

impl<'ctx, 'm> FunctionCompiler<'ctx, 'm> {
    #[inline(always)]
    pub fn builder(&self) -> &Builder<'ctx> {
        &self.builder
    }

    pub fn load_argument(&self, arg: &FunctionArgument) -> BasicValueEnum<'ctx> {
        self.ir.get_nth_param(arg.pos_id).unwrap()
    }

    pub fn add_basic_block(&self) -> BasicBlockIR<'ctx> {
        self.context().append_basic_block(self.ir, "")
    }
}

impl Function {
    pub fn compile_type<'ctx>(&self, builder: &ModuleCompiler<'ctx>) -> FunctionType<'ctx> {
        let return_type_ir = self.return_type.clone().into_ir(&builder.context());

        let arg_type_irs: Vec<BasicMetadataTypeEnum> = self
            .args
            .iter()
            .map(|arg| arg.arg_type.clone().into_ir(&builder.context()).into())
            .collect();

        return_type_ir.fn_type(&arg_type_irs, false)
    }

    pub fn compile(&self, module_compiler: &ModuleCompiler) {
        let context = module_compiler.context();
        let builder = context.create_builder();
        let function_ir = module_compiler.add_function(self);
        let function_compiler = FunctionCompiler {
            context: module_compiler.context(),
            module_compiler,
            ir: function_ir,
            builder,
        };

        let basic_block = function_compiler.add_basic_block();
        function_compiler.builder.position_at_end(basic_block);

        let entry_basic_block = self.entry_basic_block.get().unwrap();
        entry_basic_block.compile(&function_compiler);
    }
}
