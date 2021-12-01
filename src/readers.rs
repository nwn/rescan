use std::io::{BufRead, Result as IoResult};
use crate::{Scanner, Result};

/// Read values from a line of standard input.
///
/// Input is read from [`stdin`] until either the input is exhausted or a
/// newline character (`'\n'`) is read. The line, after stripping any line
/// endings (`"\n"` or `"\r\n"`), is scanned according to the given format.
///
/// See the [module-level documentation](crate) for a description of the
/// argument syntax.
///
/// [`stdin`]: std::io::stdin
#[macro_export]
macro_rules! scanln {
    ($($t:tt)+) => {{
        let stdin = std::io::stdin();
        let mut stdin_lock = stdin.lock();
        $crate::scanln_from!(&mut stdin_lock, $($t)+)
    }}
}

/// Read values from a line of input.
///
/// Input is read from `$r`, an instance of [`BufRead`], until either the input
/// is exhausted or a newline character (`'\n'`) is read. The line, after
/// stripping any line endings (`"\n"` or `"\r\n"`), is scanned according to the
/// given format.
///
/// See the [module-level documentation](crate) for a description of the
/// argument syntax.
#[macro_export]
macro_rules! scanln_from {
    ($r:expr, $($t:tt)+) => {{
        match $crate::readers::read_line($r) {
            Ok(line) => rescan::scanner!($($t)+).scan(&mut line.unwrap_or_default().as_slice()),
            Err(err) => Err($crate::error::ScanError::from(err).into()),
        }
    }}
}

/// An iterator that reads values from lines of a [`BufRead`].
///
/// This struct is created by calling [`scan_lines`](crate::Scanner::scan_lines)
/// with a `BufRead`.
pub struct LineIter<'a, Output> {
    scanner: &'a Scanner<Output>,
    reader: &'a mut dyn BufRead,
}
impl<'a, Output> LineIter<'a, Output> {
    pub(crate) fn new(scanner: &'a Scanner<Output>, reader: &'a mut dyn BufRead) -> Self {
        Self { reader, scanner }
    }
}
impl<'a, Output> Iterator for LineIter<'a, Output> {
    type Item = Result<Output>;
    fn next(&mut self) -> Option<Self::Item> {
        match read_line(self.reader).transpose() {
            Some(Ok(line)) => Some(self.scanner.scan(&mut line.as_slice())),
            Some(Err(err)) => Some(Err(crate::error::ScanError::from(err).into())),
            None => None,
        }
    }
}

/// An iterator that repeatedly reads values from a [`BufRead`].
///
/// This struct is created by calling [`scan_multiple`](crate::Scanner::scan_multiple)
/// with a `BufRead`.
pub struct ScanIter<'a, Output> {
    scanner: &'a Scanner<Output>,
    reader: &'a mut dyn BufRead,
    sep: Option<&'a str>,
    expect_sep: bool,
}
impl<'a, Output> ScanIter<'a, Output> {
    pub(crate) fn new(scanner: &'a Scanner<Output>, reader: &'a mut dyn BufRead) -> Self {
        Self { reader, scanner, sep: None, expect_sep: false }
    }
    pub(crate) fn with_separator(scanner: &'a Scanner<Output>, reader: &'a mut dyn BufRead, sep: &'a str) -> Self {
        Self { reader, scanner, sep: Some(sep), expect_sep: false }
    }
}
impl<'a, Output> Iterator for ScanIter<'a, Output> {
    type Item = Output;
    fn next(&mut self) -> Option<Self::Item> {
        if let (Some(sep), true) = (self.sep, self.expect_sep) {
            crate::internal::match_literal(self.reader, sep).ok()?;
        }
        self.expect_sep = true;
        self.scanner.scan(self.reader).ok()
    }
}

/// Read a single line from a [`BufRead`].
///
/// Reads bytes from `reader` until either the first newline character (`'\n'`)
/// or `EOF` is reached. Any error when reading from `reader` is returned as-is.
/// If `reader` has already reached the `EOF`, `Ok(None)` is returned.
#[doc(hidden)]
pub fn read_line(reader: &mut dyn BufRead) -> IoResult<Option<Vec<u8>>> {
    let mut buf = vec![];
    reader.read_until(b'\n', &mut buf)?;
    if !buf.is_empty() {
        if buf.ends_with(&[b'\n']) {
            buf.pop();
            if buf.ends_with(&[b'\r']) {
                buf.pop();
            }
        }
        Ok(Some(buf))
    } else {
        Ok(None)
    }
}

#[test]
fn line_reader() {
    let mut reader = "A\nBC\nD".as_bytes();
    assert_eq!(Some(b"A".to_vec()), read_line(&mut reader).unwrap());
    assert_eq!(Some(b"BC"[..].to_vec()), read_line(&mut reader).unwrap());
    assert_eq!(Some(b"D"[..].to_vec()), read_line(&mut reader).unwrap());
    assert_eq!(None, read_line(&mut reader).unwrap().as_ref());
}
