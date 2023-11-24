pub use crate::{error::Error, network::Network, traits::*, util};
pub use serde::{Deserialize, Serialize};
pub use std::fs;

pub type Result<T> = std::result::Result<T, crate::error::Error>;
