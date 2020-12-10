use memchr::memchr;
use std::io::{BufRead, Read, Result};

#[macro_export]
macro_rules! scanln {
    ($($t:tt)+) => {{
        let stdin = std::io::stdin();
        let mut reader = rescan::readers::LineReader::new(stdin.lock());
        let scanner = rescan::scanner!($($t)+);
        scanner(&mut reader)
    }}
}

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
    fn read(&mut self, _buf: &mut [u8]) -> Result<usize> {
        panic!("The `LineReader` type does not actually implement `Read`, and must instead by used through its `BufRead` interface.")
    }
}
impl<Buf: BufRead> BufRead for LineReader<Buf> {
    fn fill_buf(&mut self) -> Result<&[u8]> {
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
