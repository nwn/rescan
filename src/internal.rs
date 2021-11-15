use crate::error::ScanError::{self, *};
use std::io::BufRead;
use once_cell::unsync::Lazy;

pub use regex::{Regex, Error as RegexError};
pub use crate::Result;

/// The type returned by the [`scanner`] macro.
///
/// To use, pass a [`BufRead`] type into the [`scan`] function.
///
/// [`scanner`]: macro.scanner.html
/// [`BufRead`]: https://doc.rust-lang.org/std/io/trait.BufRead.html
/// [`scan`]: #method.scan
pub struct Scanner<T> {
    lazy_regexes: Lazy<Result<Vec<Regex>, RegexError>>,
    scan_fn: fn(&mut dyn BufRead, &[Regex]) -> Result<T>,
}
impl<T> Scanner<T> {
    #[doc(hidden)]
    pub fn new(regex_fn: fn() -> Result<Vec<Regex>, RegexError>, scan_fn: fn(&mut dyn BufRead, &[Regex]) -> Result<T>) -> Self {
        Self {
            lazy_regexes: Lazy::new(regex_fn),
            scan_fn,
        }
    }

    /// Attempts to read values of type `T` from the reader.
    ///
    /// This function will fail if the contents of the reader do not match the
    /// format string used to create this `Scanner`. In this case, an `Err` is
    /// returned and the reader will have advanced by an unspecified amount.
    pub fn scan(&self, reader: &mut dyn BufRead) -> Result<T> {
        let regexes = self.lazy_regexes.as_ref()?;
        (self.scan_fn)(reader, regexes)
    }
}

/// A dummy function with the same signature as that returned by a call to
/// `scanner`.
///
/// To prevent unnecessary type errors, a pointer to this function is
/// emitted in lieu of an actual scanner function when then input to the
/// `scanner` macro is invalid.
pub fn dummy<T>(_reader: &mut dyn BufRead) -> Result<T> {
    std::unimplemented!()
}

/// Attempts to read the string `lit` from the reader. If successful, the
/// reader is automatically advanced past the match. Otherwise, an error
/// results, and the reader will have advanced past some prefix of `lit`.
pub fn match_literal(reader: &mut dyn BufRead, mut lit: &str) -> Result<(), ScanError> {
    let mismatch_error = Err(ScanLiteralError(lit.into()));
    while !lit.is_empty() {
        let buf = try_read_str(reader)?;

        if lit.len() <= buf.len() {
            if buf.starts_with(lit) {
                reader.consume(lit.len());
                return Ok(());
            }
        } else if !buf.is_empty() && lit.starts_with(buf) {
            let advanced = buf.len();
            lit = &lit[advanced..];
            reader.consume(advanced);
            continue;
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
pub fn match_regex<'r>(reader: &'r mut dyn BufRead, re: &Regex) -> Result<&'r str, ScanError> {
    let buf = try_read_str(reader)?;
    if let Some(range) = re.find(buf) {
        if range.start() == 0 {
            return Ok(range.as_str());
        }
    }
    Err(ScanRegexError(re.as_str().into()))
}

/// Advance the reader by the given string. This should only be called with
/// the length of the match previously returned from `match_regex`.
pub fn advance_from_regex(reader: &mut dyn BufRead, match_len: usize) {
    reader.consume(match_len);
}

/// Returns the longest valid UTF-8 sequence from the reader, or a
/// `ScanError` if there are invalid bytes at the start.
fn try_read_str(reader: &mut dyn BufRead) -> Result<&str, ScanError> {
    let buf = reader.fill_buf()?;
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
    std::str::from_utf8(bytes).or_else(|utf8_error| {
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
    })
}

#[test]
fn longest_utf8_prefix_test() {
    let full = "ăѣ𝔠";
    assert_eq!(Ok(""), longest_utf8_prefix(&full.as_bytes()[..1]));
    assert_eq!(Ok("ă"), longest_utf8_prefix(&full.as_bytes()[..2]));
    assert_eq!(Ok("ă"), longest_utf8_prefix(&full.as_bytes()[..3]));
    assert_eq!(Ok("ăѣ"), longest_utf8_prefix(&full.as_bytes()[..4]));
    assert_eq!(Ok("ăѣ"), longest_utf8_prefix(&full.as_bytes()[..5]));
    assert_eq!(Ok("ăѣ"), longest_utf8_prefix(&full.as_bytes()[..6]));
    assert_eq!(Ok("ăѣ"), longest_utf8_prefix(&full.as_bytes()[..7]));
    assert_eq!(Ok("ăѣ𝔠"), longest_utf8_prefix(&full.as_bytes()[..8]));
}
