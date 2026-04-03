use std::collections::HashMap;
use std::ops::Deref;

use inkwell::context::Context;
use inkwell::execution_engine::JitFunction;
use inkwell::targets::TargetTriple;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::OptimizationLevel;

use crate::errors::{CompilationError, CompilationResult};
use crate::float_type::FloatType;
use crate::function::Function;
use crate::function_translator::FunctionTranslator;
use crate::function_value::FunctionValue;
use crate::integer_type::IntegerTypeSize;
use crate::module::ModuleVisitor;
use crate::translator::Translator;
use crate::types::Type;
use crate::value::Value;

type ModuleIR<'ctx> = inkwell::module::Module<'ctx>;

pub struct ModuleTranslator<'ctx> {
    parent: &'ctx Translator,
    module_ir: ModuleIR<'ctx>,
    globals: HashMap<String, Value<'ctx>>,
}

impl<'ctx> Deref for ModuleTranslator<'ctx> {
    type Target = Translator;

    fn deref(&self) -> &Self::Target {
        self.parent
    }
}

impl<'ctx> ModuleVisitor for ModuleTranslator<'ctx> {
    fn visit_function(&mut self, name: Option<&str>, function: &Function) -> CompilationResult<()> {
        let function_signature = function.signature();

        let mut arg_type_irs = Vec::<BasicMetadataTypeEnum>::new();
        for argument in &function_signature.args {
            arg_type_irs.push(self.translate_type(&argument.value_type).into());
        }

        let return_type_ir = self.translate_type(&function_signature.return_type);
        let function_type_ir = return_type_ir.fn_type(&arg_type_irs, false);
        let function_ir = self
            .module_ir
            .add_function(name.unwrap_or(""), function_type_ir, None);

        if let Some(name) = name {
            self.globals.insert(
                name.to_string(),
                FunctionValue::from_ir(function_ir, function_signature).into(),
            );
        }

        let function_translator = FunctionTranslator::new(function_ir, function_signature, self)?;
        function.visit(&function_translator)?;

        Ok(())
    }
}

impl<'ctx> ModuleTranslator<'ctx> {
    pub fn new(parent: &'ctx Translator) -> ModuleTranslator<'ctx> {
        let module_ir = parent.context().create_module("test_module");
        module_ir.set_triple(&TargetTriple::create("x86_64-pc-linux-gnu"));

        ModuleTranslator {
            parent,
            module_ir,
            globals: HashMap::new(),
        }
    }

    #[inline(always)]
    pub fn context(&self) -> &'ctx Context {
        self.parent.context()
    }

    pub fn translate_type(&self, type_spec: &Type) -> BasicTypeEnum<'ctx> {
        let context = self.context();
        match type_spec {
            Type::Bool => context.bool_type().as_basic_type_enum(),
            Type::Integer(integer_type) => {
                let type_ir = match integer_type.width {
                    IntegerTypeSize::I8 => context.i8_type(),
                    IntegerTypeSize::I16 => context.i16_type(),
                    IntegerTypeSize::I32 => context.i32_type(),
                    IntegerTypeSize::I64 => context.i64_type(),
                };
                type_ir.as_basic_type_enum()
            }
            Type::Float(float_type) => {
                let type_ir = match float_type {
                    FloatType::F32 => context.f32_type(),
                    FloatType::F64 => context.f64_type(),
                };
                type_ir.as_basic_type_enum()
            }
        }
    }

    pub fn load_value(&self, name: &str) -> CompilationResult<Value<'ctx>> {
        match self.globals.get(name) {
            Some(value) => Ok(value.clone()),
            None => Err(CompilationError::UnresolvedName(name.to_string())),
        }
    }

    pub fn run_test(&self) {
        self.module_ir.print_to_stderr();

        type TestFunc = unsafe extern "C" fn(i32, i32, i32, bool) -> i64;

        let execution_engine = self
            .module_ir
            .create_jit_execution_engine(OptimizationLevel::None)
            .unwrap();

        unsafe {
            let test_function: JitFunction<'_, TestFunc> =
                execution_engine.get_function("test").unwrap();

            let x = 1i32;
            let y = 2i32;
            let z = 3i32;
            let w = true;
            dbg!(test_function.call(x, y, z, w));
        }
    }
}
