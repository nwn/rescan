use memchr::memchr;
use std::io::{BufRead, Read, Result as IoResult};
use crate::{Scanner, Result};

#[macro_export]
macro_rules! scanln {
    ($($t:tt)+) => {{
        let stdin = std::io::stdin();
        let mut reader = rescan::readers::LineReader::new(stdin.lock());
        let scanner = rescan::scanner!($($t)+);
        scanner.scan(&mut reader)
    }}
}

#[doc(hidden)]
pub struct LineReader<Buf: BufRead> {
    inner: Buf,
    next_newline: Option<usize>,
}
impl<Buf: BufRead> LineReader<Buf> {
    pub fn new(inner: Buf) -> Self {
        Self {
            inner,
            next_newline: None,
        }
    }
}
impl<Buf: BufRead> Read for LineReader<Buf> {
    fn read(&mut self, _buf: &mut [u8]) -> IoResult<usize> {
        unimplemented!(concat!(
            "The `LineReader` type does not actually implement `Read`, ",
            "and must instead by used through its `BufRead` interface."
        ))
    }
}
impl<Buf: BufRead> BufRead for LineReader<Buf> {
    fn fill_buf(&mut self) -> IoResult<&[u8]> {
        let mut buf = self.inner.fill_buf()?;
        if let Some(newline) = memchr(b'\n', buf) {
            buf = &buf[..=newline];
            self.next_newline = Some(newline);
        }
        Ok(buf)
    }
    fn consume(&mut self, mut amt: usize) {
        if Some(amt) == self.next_newline {
            amt += 1;
            self.next_newline = None;
        }
        self.inner.consume(amt)
    }
}

#[macro_export]
macro_rules! scan_lines {
    ($r:expr, $($t:tt)+) => {{
        let reader = $r;
        let scanner = rescan::scanner!($($t)+);
        rescan::readers::LineIter::new(reader, scanner)
    }}
}

pub struct LineIter<Buf: BufRead, Output> {
    scanner: crate::Scanner<Output>,
    reader: LineReader<Buf>,
}
impl<Buf: BufRead, Output> LineIter<Buf, Output> {
    pub fn new(reader: Buf, scanner: Scanner<Output>) -> Self {
        Self {
            reader: LineReader::new(reader),
            scanner,
        }
    }
}
impl<Buf: BufRead, Output> Iterator for LineIter<Buf, Output> {
    type Item = Result<Output>;
    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(buf) = self.reader.inner.fill_buf() {
            if buf.is_empty() {
                // Reached EOF
                return None;
            }
        }
        Some(self.scanner.scan(&mut self.reader))
    }
}
