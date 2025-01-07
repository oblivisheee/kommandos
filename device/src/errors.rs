use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Candle-core error: {0}")]
    CandleError(#[from] candle_core::Error),
}
