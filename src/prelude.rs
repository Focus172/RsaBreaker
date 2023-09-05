pub use crate::{
    error::Error,
    util
};

pub type Result<T> = std::result::Result<T, crate::error::Error>;
