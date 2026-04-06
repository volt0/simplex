#[derive(Clone, PartialEq, PartialOrd)]
pub enum FloatType {
    F32,
    F64,
}

impl FloatType {
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
