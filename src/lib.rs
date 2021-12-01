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
///
/// Implementing this trait for a type enables it to be given as an argument to
/// any of the scanning macros. When scanning a capture, `Scan`'s [`scan`]
/// method will be invoked on an input substring exactly matching the specified
/// regular expression.
///
/// The `Output` associated type is the type produced by `scan` when successful.
/// This will usually be `Self`, but can be any type. This is useful to enable
/// scanning of foreign types (circumventing the orphan rule), and to provide
/// alternative parsing rules for a given type. (See, for example, [`Hex`].)
///
/// The `Error` associated type is the returned failure value of `scan`. It can
/// be any type implementing the [`std::error::Error`] trait.
///
/// [`scan`]: Self::scan
/// [`Hex`]: crate::Hex
pub trait Scan {
    type Output: Sized;
    type Error: StdError;

    /// Parse a string to return a value of type `Self::Output`.
    fn scan(text: &str) -> Result<Self::Output, Self::Error>;
}

/// Parse a value from a string with a default regular expression.
///
/// This trait extends [`Scan`] with a default regular expression. This allows
/// `DefaultScan` types to appear in arguments to scanning macros without an
/// explicit regex literal.
///
/// For example, to parse a `bool` without this trait, we would need to specify
/// something like:
/// ```rust
/// # use rescan::scanner;
/// scanner!("{}", "true|false" as bool);
/// ```
/// every time. Instead, since `bool` implements `DefaultScan`, we can shorten
/// this to:
/// ```rust
/// # use rescan::scanner;
/// scanner!("{}", bool);
/// ```
/// Of course, it is always possible to override the default regex with an
/// explicit one when desired.
///
/// [`Scan`]: crate::Scan
pub trait DefaultScan: Scan {
    /// The default regex to use in a scanning macro when none is specified.
    const DEFAULT_REGEX: &'static str;
}
