mod internal;
pub mod error;
mod example;
mod impls;
mod scanner;
pub mod readers;
#[doc(hidden)]
pub mod _rescan_internal {
    pub use crate::internal::*;
}

pub use rescan_macros::scanner;
pub use scanner::Scanner;
pub use error::{Error, Result};
pub use impls::{Binary, Octal, Hex};
use std::error::Error as StdError;

/// Parse a value from a string.
pub trait Scan {
    type Output: Sized;
    type Error: StdError;
    fn scan(text: &str) -> Result<Self::Output, Self::Error>;
}
/// Parse a value from a string with a default regular expression.
pub trait DefaultScan: Scan {
    const DEFAULT_REGEX: &'static str;
}
