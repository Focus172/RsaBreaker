use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Io failure: {0}")]
    Io(#[from] io::Error),
    #[error("Failed to serialized the Network: `{0}`")]
    Creation(#[from] json::Error),
    #[error("Generic Error: `{0}`")]
    Static(String),
}

impl Error {
    pub fn generic(thing: String) -> Self {
        Self::Static(thing)
    }
}
