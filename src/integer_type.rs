use inkwell::context::Context;
use inkwell::types::IntType;

#[derive(Clone, PartialEq, PartialOrd)]
pub enum IntegerTypeWidth {
    I8,
    I16,
    I32,
    I64,
}

#[derive(Clone)]
pub struct IntegerType<'ctx> {
    ir: IntType<'ctx>,
    is_signed: bool,
}

impl<'ctx> IntegerType<'ctx> {
    #[inline]
    pub fn new(ir: IntType<'ctx>, is_signed: bool) -> Self {
        Self { ir, is_signed }
    }

    pub fn from_spec(context: &'ctx Context, width: IntegerTypeWidth, is_signed: bool) -> Self {
        let ir = match width {
            IntegerTypeWidth::I8 => context.i8_type(),
            IntegerTypeWidth::I16 => context.i16_type(),
            IntegerTypeWidth::I32 => context.i32_type(),
            IntegerTypeWidth::I64 => context.i64_type(),
        };
        Self { ir, is_signed }
    }

    #[inline]
    pub fn is_signed(&self) -> bool {
        self.is_signed
    }

    #[inline]
    pub fn ir(&self) -> &IntType<'ctx> {
        &self.ir
    }

    #[inline]
    pub fn bit_width(&self) -> u32 {
        self.ir.get_bit_width()
    }

    pub fn is_compatible(&self, other_type: &IntegerType<'ctx>) -> bool {
        if self.is_signed == other_type.is_signed {
            self.bit_width() <= other_type.bit_width()
        } else if other_type.is_signed && !self.is_signed {
            self.bit_width() < other_type.bit_width()
        } else {
            false
        }
    }

    // pub fn combine_with(&self, other_type: Type<'ctx>) -> CompilationResult<Self> {
    //     Ok(match other_type {
    //         Type::Integer(other_type) => {
    //             if self.is_signed == other_type.is_signed {
    //                 if other_type.width() > self.width() {
    //                     other_type.clone()
    //                 } else {
    //                     self.clone()
    //                 }
    //             } else if other_type.is_signed && other_type.width() > self.width() {
    //                 other_type.clone()
    //             } else if self.is_signed && self.width() > other_type.width() {
    //                 self.clone()
    //             } else {
    //                 return Err(CompilationError::TypeMismatch);
    //             }
    //         }
    //         Type::Bool => self.clone(),
    //         _ => return Err(CompilationError::TypeMismatch),
    //     })
    // }
}
