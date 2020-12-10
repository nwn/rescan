mod error;
mod int;

use regex::Regex;
pub use rescan_format::format;
pub use error::{ScanError, Error, Result};
use std::error::Error as StdError;
use regex::Error as RegexError;

pub trait Scan: Sized {
    const REGEX: &'static str;
    type Error: StdError;
    fn scan(text: &str) -> Result<Self, Self::Error>;
}

// The `rescan-format` macro will produce an adhoc type implementing this trait.
// Users might be able to hold either an `impl Scanner<T>` or a `dyn Scanner<T>`
// to store the scanner for use in multiple places. (Should look into what trait
// objects are and what object safety means.) The usual use case will be to
// create and immediately scan within the macro output, though. The `Output`
// type will be either a unit, a single bare type, or a tuple of types. Any
// regexes in this scan will be constructed and validated immediately. The
// generated `scan` function will simply pass these into the `internal` matching
// functions (`match_literal` and `match_regex`).
pub trait Scanner {
    type Output;
    fn scan(reader: &mut impl std::io::BufRead) -> Result<Self::Output>;
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
    rescan_format::parse_to_abstract!("A:{} B:{4:2} C:{2} D:{3:0} E:{:name} F:{5:id}", u32, "-?\\d+" as i8, u64, name = u16, id = "[0-9]" as u32);
}


mod matching {
    use super::ScanError::{self, *};
    use super::Regex;

    /// Attempts to read the string `lit` from the reader. If successful, the
    /// reader is automatically advanced past the match. Otherwise, an error
    /// results, and the reader will have advanced past some prefix of `lit`.
    pub fn match_literal(reader: &mut impl std::io::BufRead, mut lit: &'static str) -> Result<(), ScanError> {
        let mismatch_error = Err(ScanLiteralError(lit));
        while !lit.is_empty() {
            let buf = try_read_str(reader)?;

            if lit.len() <= buf.len() {
                if buf.starts_with(lit) {
                    reader.consume(lit.len());
                    return Ok(());
                }
            } else if !buf.is_empty() {
                if lit.starts_with(buf) {
                    let advanced = buf.len();
                    lit = &lit[advanced..];
                    reader.consume(advanced);
                    continue;
                }
            }
            return mismatch_error;
        }
        Ok(())
    }

    /// Attempts to match the given regex at the start of the reader. If
    /// successful, the matched portion of the string is returned. Otherwise, an
    /// error is returned. In any case, the reader is not advanced---this must
    /// be done manually by calling the `advance_from_regex` function with the
    /// length of the match from this function.
    pub fn match_regex<'r>(reader: &'r mut impl std::io::BufRead, re: &'static Regex) -> Result<&'r str, ScanError> {
        let buf = try_read_str(reader)?;
        if let Some(range) = re.find(buf) {
            if range.start() == 0 {
                return Ok(range.as_str());
            }
        }
        Err(ScanRegexError(re.as_str()))
    }

    /// Advance the reader by the given string. This should only be called with
    /// the length of the match previously returned from `match_regex`.
    pub fn advance_from_regex(reader: &mut impl std::io::BufRead, match_len: usize) {
        reader.consume(match_len);
    }

    /// Returns the longest valid UTF-8 sequence from the reader, or a
    /// `ScanError` if there are invalid bytes at the start.
    fn try_read_str(reader: &mut impl std::io::BufRead) -> Result<&str, ScanError> {
        let buf = reader.fill_buf().unwrap();
        longest_utf8_prefix(buf).map_err(|error_bytes| {
            let len = error_bytes.len();
            let mut bytes = [0; 4];
            bytes[..len].copy_from_slice(error_bytes);
            ScanDecodeError { bytes, len }
        })
    }

    /// Converts as much of a slice of bytes to a string slice as possible.
    ///
    /// A UTF-8 string being buffered as bytes may not terminate with a complete
    /// UTF-8 code point sequence. `longest_utf8_prefix` extracts the valid portion
    /// of the slice.
    ///
    /// If there are erroneous bytes at the start of the slice, they will be
    /// returned as an `Err` instead.
    fn longest_utf8_prefix(bytes: &[u8]) -> Result<&str, &[u8]> {
        match std::str::from_utf8(bytes) {
            Ok(str) => Ok(str),
            Err(utf8_error) => {
                match (utf8_error.valid_up_to(), utf8_error.error_len()) {
                    (0, Some(error_len)) => Err(&bytes[..error_len]),
                    (valid_up_to, _) => {
                        // SAFETY: The `Utf8Error::valid_up_to()` function guarantees
                        // that the range up to that point is valid UTF-8.
                        unsafe {
                            Ok(std::str::from_utf8_unchecked(&bytes[..valid_up_to]))
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn longest_utf8_prefix_test() {
        let full = "ăѣ𝔠";
        assert_eq!("", longest_utf8_prefix(&full.as_bytes()[..1]));
        assert_eq!("ă", longest_utf8_prefix(&full.as_bytes()[..2]));
        assert_eq!("ă", longest_utf8_prefix(&full.as_bytes()[..3]));
        assert_eq!("ăѣ", longest_utf8_prefix(&full.as_bytes()[..4]));
        assert_eq!("ăѣ", longest_utf8_prefix(&full.as_bytes()[..5]));
        assert_eq!("ăѣ", longest_utf8_prefix(&full.as_bytes()[..6]));
        assert_eq!("ăѣ", longest_utf8_prefix(&full.as_bytes()[..7]));
        assert_eq!("ăѣ𝔠", longest_utf8_prefix(&full.as_bytes()[..8]));
    }
}
