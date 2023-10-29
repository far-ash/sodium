pub mod error;

pub use crate::error::error::Error;
use std::result;

pub type Result<T> = result::Result<T, Error>;
