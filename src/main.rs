use inkwell::basic_block::BasicBlock;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::module::Module as ModuleIR;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum, FunctionType};
use inkwell::values::{BasicValueEnum, FunctionValue};
use inkwell::OptimizationLevel;
use slotmap::{DefaultKey, SlotMap};
use std::cell::{OnceCell, RefCell};
use std::ops::Deref;
use std::rc::Rc;

// mod ast;
//
// mod grammar {
//     include!(concat!(env!("OUT_DIR"), "/grammar.rs"));
// }

type SumFunc = unsafe extern "C" fn(u64, u64, u64) -> u64;

// struct ExpressionBuilder<'ctx, 'm, 'f> {}

struct BasicBlockBuilder<'ctx, 'm, 'f> {
    context: &'ctx Context,
    function_builder: &'f FunctionBuilder<'ctx, 'm>,
    ir_builder: Builder<'ctx>,
    block: BasicBlock<'ctx>,
}

impl<'ctx, 'm, 'f> Deref for BasicBlockBuilder<'ctx, 'm, 'f> {
    type Target = FunctionBuilder<'ctx, 'm>;

    fn deref(&self) -> &Self::Target {
        self.function_builder
    }
}

struct FunctionBuilder<'ctx, 'm> {
    context: &'ctx Context,
    module_builder: &'m ModuleBuilder<'ctx>,
    ir: FunctionValue<'ctx>,
}

impl<'ctx, 'm> Deref for FunctionBuilder<'ctx, 'm> {
    type Target = ModuleBuilder<'ctx>;

    fn deref(&self) -> &Self::Target {
        self.module_builder
    }
}

impl<'ctx, 'm> FunctionBuilder<'ctx, 'm> {
    fn add_basic_block(&self) -> BasicBlockBuilder {
        todo!()
    }

    fn test(&self) {
        let builder = self.context.create_builder();

        let basic_block = self.context.append_basic_block(self.ir, "entry");
        builder.position_at_end(basic_block);

        let x_id = self.add_value(self.ir.get_nth_param(0).unwrap());
        let x = self.get_value(x_id).unwrap().into_int_value();
        let y = self.ir.get_nth_param(1).unwrap().into_int_value();
        let z = self.ir.get_nth_param(2).unwrap().into_int_value();

        let sum = builder.build_int_add(x, y, "sum").unwrap();
        let sum = builder.build_int_add(sum, z, "sum").unwrap();

        builder.build_return(Some(&sum)).unwrap();
    }
}

struct ModuleBuilder<'ctx> {
    context: &'ctx Context,
    ir: ModuleIR<'ctx>,
    values: RefCell<SlotMap<DefaultKey, BasicValueEnum<'ctx>>>,
}

impl<'ctx> ModuleBuilder<'ctx> {
    fn add_value(&self, value: BasicValueEnum<'ctx>) -> DefaultKey {
        self.values.borrow_mut().insert(value)
    }

    fn get_value(&self, id: DefaultKey) -> Option<BasicValueEnum<'ctx>> {
        self.values.borrow().get(id).cloned()
    }

    fn add_function(&self, function: &Function) -> FunctionBuilder<'ctx, '_> {
        let fn_type = function.compile_type(self);
        let function_ir = self.ir.add_function("sum", fn_type, None);

        FunctionBuilder {
            context: &self.context,
            module_builder: self,
            ir: function_ir,
        }
    }
}

#[derive(Clone)]
enum TypeSpec {
    I64,
}

impl TypeSpec {
    fn into_ir(self, context: &Context) -> BasicTypeEnum {
        match self {
            TypeSpec::I64 => context.i64_type().as_basic_type_enum(),
        }
    }
}

struct FunctionArgument {
    arg_type: TypeSpec,
    ir_id: OnceCell<DefaultKey>,
}

struct Function {
    args: Vec<FunctionArgument>,
    return_type: TypeSpec,
}

impl Function {
    fn compile(&self, builder: &FunctionBuilder) {
        for (pos_id, arg) in self.args.iter().enumerate() {
            let arg_ir = builder.add_value(builder.ir.get_nth_param(pos_id as u32).unwrap());
            arg.ir_id.set(arg_ir).unwrap();
        }

        builder.test()
    }

    fn compile_type<'ctx>(&self, builder: &ModuleBuilder<'ctx>) -> FunctionType<'ctx> {
        let return_type_ir = self.return_type.clone().into_ir(&builder.context);

        let arg_type_irs: Vec<BasicMetadataTypeEnum> = self
            .args
            .iter()
            .map(|arg| arg.arg_type.clone().into_ir(&builder.context).into())
            .collect();

        return_type_ir.fn_type(&arg_type_irs, false)
    }
}

struct Module {
    functions: Vec<Rc<Function>>,
}

impl Module {
    fn compile(&self, builder: &ModuleBuilder) {
        for function in self.functions.iter().cloned() {
            let builder = builder.add_function(function.as_ref());
            function.compile(&builder)
        }

        let execution_engine = builder
            .ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let sum: JitFunction<SumFunc> = execution_engine.get_function("sum").unwrap();
            let x = 1u64;
            let y = 2u64;
            let z = 3u64;
            dbg!(sum.call(x, y, z));
        }
    }
}

fn main() {
    let module = Rc::new(Module {
        functions: vec![Rc::new(Function {
            args: vec![
                FunctionArgument {
                    arg_type: TypeSpec::I64,
                    ir_id: Default::default(),
                },
                FunctionArgument {
                    arg_type: TypeSpec::I64,
                    ir_id: Default::default(),
                },
                FunctionArgument {
                    arg_type: TypeSpec::I64,
                    ir_id: Default::default(),
                },
            ],
            return_type: TypeSpec::I64,
        })],
    });

    let context = Context::create();
    let module_ir = context.create_module("sum");
    let module_builder = ModuleBuilder {
        context: &context,
        ir: module_ir,
        values: Default::default(),
    };
    module.compile(&module_builder);
}
