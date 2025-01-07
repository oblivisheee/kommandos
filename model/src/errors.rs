use std::io::Error as IoError;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum Error {
    #[error("Candle-core error: {0}")]
    CandleError(#[from] candle_core::Error),
    #[error("IO error: {0}")]
    IoError(#[from] IoError),
}
