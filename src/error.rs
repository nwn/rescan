use regex::Error as RegexError;
use std::error::Error as StdError;

pub type Result<T, E = Error> = std::result::Result<T, E>;

// The error type returned by the top-level macro. Errors can occur either
// when compiling the regexes or when parsing a capture fails.
#[derive(Debug)]
pub enum Error {
    RegexError(RegexError),
    ScanError(ScanError),
    ParseError(Box<dyn StdError>),
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
impl From<RegexError> for Error {
    fn from(error: RegexError) -> Self {
        Self::RegexError(error)
    }
}
impl From<ScanError> for Error {
    fn from(error: ScanError) -> Self {
        Self::ScanError(error)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ScanError {
    ScanDecodeError {
        len: usize,
        bytes: [u8; 4],
    },
    ScanLiteralError(&'static str),
    ScanRegexError(&'static str),
}
impl std::fmt::Display for ScanError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::ScanDecodeError { bytes, len } => {
                write!(f, "invalid UTF-8 sequence: {:x?}", &bytes[..len])
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
