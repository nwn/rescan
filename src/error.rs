use regex::Error as RegexError;
use std::error::Error as StdError;
use std::io::Error as IoError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

/// The error type returned by most of the scanning functions.
///
/// Errors can occur either when compiling the regexes or when parsing a capture
/// fails.
#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Error returned when a regular expression in a scanner fails to compile.
    RegexError(RegexError),
    /// Error returned when I/O fails or when the input does not match the
    /// expected pattern.
    ScanError(ScanError),
    /// Error returned when the [`Scan::scan`](crate::Scan::scan) function fails.
    ParseError(Box<dyn StdError>),
}
impl Error {
    pub fn from_parse_error(error: impl StdError + 'static) -> Self {
        Self::ParseError(Box::new(error))
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RegexError(error) => error.fmt(f),
            Self::ScanError(error) => error.fmt(f),
            Self::ParseError(error) => error.fmt(f),
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::RegexError(error) => Some(error),
            Self::ScanError(error) => Some(error),
            Self::ParseError(error) => Some(error.as_ref()),
        }
    }
}
impl From<&RegexError> for Error {
    fn from(error: &RegexError) -> Self {
        Self::RegexError(error.clone())
    }
}
impl From<ScanError> for Error {
    fn from(error: ScanError) -> Self {
        Self::ScanError(error)
    }
}

/// Error type indicating either an I/O error or failure to match input with a
/// scanning pattern.
///
/// There are (currently) four cases handled by this type:
/// - `ScanIoError` signals the failure of an I/O operation. The original error
///   is encapsulated.
/// - `ScanDecodeError` indicates that the byte stream contained invalid UTF-8
///   characters. The byte sequence (of length up to 4) containing the
///   unexpected byte can be obtained from this variant.
/// - `ScanLiteralError` indicates that the input did not match a literal
///   portion of the format string. The expected string is returned.
/// - `ScanRegexError` indicates that the input did not match the regex
///   corresponding to a capture in the format string. The expected regex is
///   returned as a string.
#[derive(Debug)]
#[non_exhaustive]
pub enum ScanError {
    ScanIoError(IoError),
    ScanDecodeError(Utf8Error),
    ScanLiteralError(String),
    ScanRegexError(String),
}
impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::ScanIoError(io_error) => {
                io_error.fmt(f)
            }
            Self::ScanDecodeError(utf8_error) => {
                write!(f, "invalid UTF-8 sequence: {:x?}", utf8_error.error_bytes())
            }
            Self::ScanLiteralError(lit) => {
                write!(f, "input text does not match literal \"{}\"", lit)
            }
            Self::ScanRegexError(regex) => {
                write!(f, "input text does not match regex \"{}\"", regex)
            }
        }
    }
}
impl StdError for ScanError {}
impl From<IoError> for ScanError {
    fn from(error: IoError) -> Self {
        Self::ScanIoError(error)
    }
}
impl From<Utf8Error> for ScanError {
    fn from(error: Utf8Error) -> Self {
        Self::ScanDecodeError(error)
    }
}

/// Error type resulting from invalid UTF-8 characters in a byte stream.
///
/// The byte sequence containing the unexpected byte can be obtained using the
/// [`error_bytes`](Self::error_bytes) method.
#[derive(Debug)]
pub struct Utf8Error {
    len: u8,
    bytes: [u8; 4],
}
impl Utf8Error {
    /// Create a new `Utf8Error` from a sequence of bytes.
    ///
    /// Panics if `error_bytes.len() > 4`.
    pub(crate) fn new(error_bytes: &[u8]) -> Self {
        assert!(error_bytes.len() <= 4, "UTF-8 byte sequences contain at most 4 bytes");
        let mut bytes = [0; 4];
        bytes[..error_bytes.len()].copy_from_slice(error_bytes);
        Self {
            len: error_bytes.len() as _,
            bytes,
        }
    }

    /// Returns the offending byte sequence.
    ///
    /// The returned slice will have length at most 4.
    pub fn error_bytes(&self) -> &[u8] {
        &self.bytes[..self.len as usize]
    }
}
