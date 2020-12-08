mod int;

use regex::{Regex, Error as RegexError};
pub use rescan_format::format;
use std::error::Error as StdError;

pub trait Scan: Sized {
    const REGEX: &'static str;
    type Error: StdError;
    fn scan(text: &str) -> Result<Self, Self::Error>;
}

// The contents of this function are roughly what the output of the `scan`
// macro should eventually look like.
fn test() {
    use internal::*;
    static COMPILED: Lazy<Result<FormatScanner, RegexError>> = Lazy::new(|| {
        let format_spec = format!("{test} this is a {test\\{string that should{2,4}} work }} just {{ fine");
        format_spec.compile()
    });
    // COMPILED.scan($BufRead)
}

// The error type returned by the top-level macro. Errors can occur either
// when compiling the regexes or when parsing a capture fails.
#[derive(Debug)]
enum Error {
    RegexError(RegexError),
    ParseError(Box<dyn StdError>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::RegexError(error) => error.fmt(f),
            Self::ParseError(error) => error.fmt(f),
        }
    }
}
impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Self::RegexError(error) => Some(error),
            Self::ParseError(error) => Some(error.as_ref()),
        }
    }
}
impl From<regex::Error> for Error {
    fn from(error: regex::Error) -> Self {
        Self::RegexError(error)
    }
}
type Result<T, E = Error> = std::result::Result<T, E>;

#[doc(hidden)]
pub mod internal {
    pub use once_cell::sync::Lazy;

    pub struct FormatScanner {

    }

    struct Capture {
        seg_idx: usize,
    }

    // The trait that users implement to make a type scannable.
    trait Scannable { }
    // A subtype of `Scannable` that also allows omission of a regex.
    trait DefaultScannable: Scannable {
        const DEFAULT_REGEX: &'static str;
    }

    // The type representing a single argument to the `format!` macro. The macro will
    // output expressions constructing this type.
    struct ScanRule<Target: Scannable> {
        regex: &'static str,
        _phantom: std::marker::PhantomData<Target>,
    }
    impl<Target: Scannable> ScanRule<Target> {
        fn new(regex: &'static str) -> Self {
            Self {
                regex,
                _phantom: std::marker::PhantomData,
            }
        }
    }
    impl<Target: DefaultScannable> ScanRule<Target> {
        fn default() -> Self {
            Self {
                regex: Target::DEFAULT_REGEX,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct FormatSpec {
        segments: Vec<Segment<&'static str, &'static str>>,
    }
    impl FormatSpec {
        pub fn new(segments: Vec<Segment<&'static str, &'static str>>) -> Self {
            Self {
                segments
            }
        }
        pub fn compile(self) -> Result<FormatScanner, crate::RegexError> {
            todo!()
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Segment<Lit, Cap> {
        Literal(Lit),
        Capture(Cap),
    }

    // This may prove useful for proper type inference if we need to emit
    // dummy code due to an error.
    pub fn todo<T>() -> T {
        ::core::todo!()
    }
}

#[test]
fn test_abstract_parsing() {
    rescan_format::parse_to_abstract!("{}", u32);
    rescan_format::parse_to_abstract!("{:0}", "-?\\d+" as i8);
    rescan_format::parse_to_abstract!("{_}", "[+-*/]" as _);
    rescan_format::parse_to_abstract!("{}", u64);
    rescan_format::parse_to_abstract!("{0:name}", name = u16);
    rescan_format::parse_to_abstract!("{:id}", id = "[0-9]" as u32);
    rescan_format::parse_to_abstract!("A:{} B:{2:2} C:{1} D:{3:0} E:{:name} F:{5:id}", u32, "-?\\d+" as i8, u64, name = u16, id = "[0-9]" as u32);
}
