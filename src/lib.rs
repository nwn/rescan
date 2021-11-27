#[doc(hidden)]
pub mod internal;
mod error;
mod example;
mod impls;
pub mod partial_result;

#[cfg(feature = "readers")]
pub mod readers;

pub use rescan_macros::scanner;
pub use internal::Scanner;
pub use error::{ScanError, Error, Result};
pub use partial_result::PartialResult;
use std::error::Error as StdError;

/// Parse a value from a string.
pub trait Scan: Sized {
    type Error: StdError;
    fn scan(text: &str) -> Result<Self, Self::Error>;
}
/// Parse a value from a string with a default regular expression.
pub trait DefaultScan: Scan {
    const DEFAULT_REGEX: &'static str;
}
