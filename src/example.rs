use regex::{Error, bytes::Regex};
use once_cell::sync::Lazy;

fn main() {
    let lit_0 = "One might expect ";
    let lit_2 = " to have at least ";
    let lit_4 = ".";

    static CAP_1: Lazy<Result<Regex, Error>> = Lazy::new(|| {
        Regex::new(r"[[:alpha:]]+\s[[:alpha:]]+")
    });
    static CAP_3: Lazy<Result<Regex, Error>> = Lazy::new(|| {
        Regex::new(r"[[:digit:]]+\s[[:alpha:]]+")
    });

    let s = String::from("One might expect most people to have at least 4 fingers.");
    let mut s = s.as_bytes();
    println!("{}", match_literal(&mut s, lit_0));
    println!("{}", match_regex(&mut s, CAP_1.as_ref().unwrap()));
    println!("{}", match_literal(&mut s, lit_2));
    println!("{}", match_regex(&mut s, CAP_3.as_ref().unwrap()));
    println!("{}", match_literal(&mut s, lit_4));
    println!("{}", s.is_empty());
}

fn match_literal(reader: &mut impl std::io::BufRead, mut lit: &'static str) -> Result<(), ScanError> {
    let mismatch_error = ScanError::ScanLiteralError(lit);
    while !lit.is_empty() {
        let buf = reader.fill_buf().unwrap();
        let buf = longest_utf8_prefix(buf).map_err(|error_bytes| {
            let len = error_bytes.len();
            let mut bytes = [0; 4];
            bytes[..len].copy_from_slice(error_bytes);
            ScanError::ScanDecodeError { bytes, len }
        }?;

        return if lit.len() <= buf.len() {
            if buf.starts_with(lit) {
                reader.consume(lit.len());
                Ok(())
            } else {
                mismatch_error
            }
        } else if !buf.is_empty() {
            if lit.starts_with(buf) {
                let advanced = buf.len();
                lit = &lit[advanced..];
                reader.consume(advanced);
                continue;
            } else {
                mismatch_error
            }
        } else {
            mismatch_error
        };
    }
    Ok(())
}

fn match_regex(reader: &mut impl std::io::BufRead, re: &'static Regex) -> Result<&str, ScanError> {
    let mismatch_error = ScanError::ScanRegexError(re.as_str());

    let buf = reader.fill_buf().unwrap();
    let buf = longest_utf8_prefix(buf).map_err(|error_bytes| {
        let len = error_bytes.len();
        let mut bytes = [0; 4];
        bytes[..len].copy_from_slice(error_bytes);
        ScanError::ScanDecodeError { bytes, len }
    }?;

    if let Some(range) = re.find(buf) {
        if range.start() == 0 {
            return Ok(range.as_str());
        }
    }

    Err(ScanError::ScanRegexError(re.as_str()))
}

fn advance_from_regex(reader: &mut impl std::io::BufRead, matched: &str) {
    reader.consume(matched.len());
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
        Ok(str) => str,
        Err(utf8_error) => {
            match (utf8_error.valid_up_to(), utf8_error.error_len()) {
                (0, Some(error_len)) => Err(&bytes[..error_len]),
                (valid_up_to, _) => {
                    // SAFETY: The `Utf8Error::valid_up_to()` function guarantees
                    // that the range up to that point is valid UTF-8.
                    unsafe {
                        std::str::from_utf8_unchecked(&bytes[..valid_up_to])
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
