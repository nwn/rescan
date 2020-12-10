#[doc(hidden)]
pub mod internal;
mod error;
mod example;
// mod int;

pub use rescan_macros::scanner;
pub use error::{ScanError, Error, Result};
use std::error::Error as StdError;
use std::io::BufRead;

/// The type returned by the `scanner` macro.
pub type Scanner<B: BufRead, T> = fn(&mut B) -> Result<T>;

/// Parse a value from a string.
pub trait Scan: Sized {
    type Error: StdError;
    fn scan(text: &str) -> Result<Self, Self::Error>;
}
/// Parse a value from a string with a default regular expression.
pub trait DefaultScan: Scan {
    const DEFAULT_REGEX: &'static str;
}

impl Scan for String {
    type Error = std::str::ParseBoolError;
    fn scan(str: &str) -> Result<Self, Self::Error> {
        Ok(str.into())
    }
}
