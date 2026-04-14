use inkwell::context::Context;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum FloatTypeWidth {
    F32,
    F64,
}

pub type FloatTypeIR<'ctx> = inkwell::types::FloatType<'ctx>;

#[derive(Clone)]
pub struct FloatType<'ctx> {
    ir: FloatTypeIR<'ctx>,
}

impl<'ctx> FloatType<'ctx> {
    pub fn from_spec(context: &'ctx Context, width: FloatTypeWidth) -> Self {
        Self {
            ir: match width {
                FloatTypeWidth::F32 => context.f32_type(),
                FloatTypeWidth::F64 => context.f64_type(),
            },
        }
    }

    #[inline]
    pub fn ir(&self) -> &FloatTypeIR<'ctx> {
        &self.ir
    }

    #[inline]
    pub fn bit_width(&self) -> u32 {
        self.ir.get_bit_width()
    }

    // pub fn combine_with(&self, other_type: Type<'ctx>) -> CompilationResult<Self> {
    //     match other_type {
    //         Type::Float(other_type) => {
    //             if self.width() > other_type.width() {
    //                 Ok(self.clone())
    //             } else {
    //                 Ok(other_type.clone())
    //             }
    //         }
    //         _ => Err(CompilationError::TypeMismatch),
    //     }
    // }
}
