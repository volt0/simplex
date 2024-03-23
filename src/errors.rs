use inkwell::builder::BuilderError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CompilationError {
    #[error("IR builder error")]
    BuilderError(#[from] BuilderError),
}
