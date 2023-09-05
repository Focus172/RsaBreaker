use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io failure: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to serialized the Network: `{0}`")]
    Creation(#[from] json::Error),
    // #[error(transparent)]
    // Static
}
