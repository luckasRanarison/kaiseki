use bincode::error::DecodeError;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{0}")]
    DecodeError(#[from] DecodeError),
    #[error("{0}")]
    FstBuilderError(#[from] fst::Error),
}
