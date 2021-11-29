use std::io::BufRead;
use once_cell::unsync::Lazy;

pub use regex::{Regex, Error as RegexError};
pub use crate::Result;
pub use crate::readers::{LineIter, ScanIter};

/// The type returned by the [`scanner!`] macro.
///
/// To use this type, invoke [`scan`] or [`scan_lines`] with an instance of
/// [`BufRead`].
///
/// [`scanner!`]: crate::scanner!
/// [`scan`]: Self::scan
/// [`scan_lines`]: Self::scan_lines
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

    /// Returns an iterator that attempts to read values from lines of input.
    ///
    /// The iterator will yield instances of [`Result<T>`](crate::Result), the
    /// result of calling [`scan`](Self::scan) on a single line of input from `reader`.
    ///
    /// Lines are read from `reader` as sequences of bytes that end either at
    /// the first newline character (`'\n'`), or when `reader` has been
    /// exhausted. Line endings (`"\n"` or `"\r\n"`) are stripped before a line
    /// is scanned.
    pub fn scan_lines<'a>(&'a self, reader: &'a mut dyn BufRead) -> LineIter<'a, T> {
        LineIter::new(self, reader)
    }

    /// Returns an iterator that attempts to read values from lines of input.
    ///
    /// The iterator will repeatedly attempt to [`scan`](Self::scan) from `reader`.
    /// The iterator terminates at the first unsuccessful scan.
    pub fn scan_multiple<'a>(&'a self, reader: &'a mut dyn BufRead) -> ScanIter<'a, T> {
        ScanIter::new(self, reader)
    }

    /// Returns an iterator that attempts to read values from lines of input.
    ///
    /// The iterator will repeatedly attempt to [`scan`](Self::scan) from `reader`,
    /// expecting the literal `sep` to appear between each occurrence. The iterator
    /// terminates at the first unsuccessful scan.
    pub fn scan_multiple_with_separator<'a>(&'a self, reader: &'a mut dyn BufRead, sep: &'a str) -> ScanIter<'a, T> {
        ScanIter::with_separator(self, reader, sep)
    }
}
