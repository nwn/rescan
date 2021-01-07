#[doc(hidden)]
pub mod internal;
mod error;
mod example;
mod impls;

#[cfg(feature = "readers")]
pub mod readers;

pub use rescan_macros::{scanner, static_scanner};
pub use error::{ScanError, Error, Result};
use std::error::Error as StdError;
use std::io::BufRead;

/// The type returned by the `scanner` macro.
pub type Scanner<T> = fn(&mut dyn BufRead) -> Result<T>;

/// Parse a value from a string.
pub trait Scan: Sized {
    type Error: StdError;
    fn scan(text: &str) -> Result<Self, Self::Error>;
}
/// Parse a value from a string with a default regular expression.
pub trait DefaultScan: Scan {
    const DEFAULT_REGEX: &'static str;
}
