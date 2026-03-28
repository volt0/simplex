use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("type mismatch")]
    TypeMismatch,

    #[error("cannot find `{0}` in this scope")]
    UnresolvedName(String),

    #[error("invalid operation")]
    InvalidOperation,

    #[error("builder internal error: {0:?}")]
    BuilderError(#[from] inkwell::builder::BuilderError),
}
