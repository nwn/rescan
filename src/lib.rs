mod int;

use regex::Regex;
pub use rescan_format::format;

pub trait Scan: Sized {
    const REGEX: &'static str;
    type Error: std::error::Error;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error>;
}

// The contents of this function are roughly what the output of the `scan`
// macro should eventually look like.
fn test() {
    use internal::*;
    static COMPILED: Lazy<FormatScanner> = Lazy::new(|| {
        let format_spec = format!("{test} this is a {test\\{string that should{2,4}} work }} just {{ fine");
        format_spec.compile()
    });
    // COMPILED.scan($BufRead)
}

#[doc(hidden)]
pub mod internal {
    pub use once_cell::sync::Lazy;

    pub struct FormatScanner;

    #[derive(Debug, PartialEq, Eq)]
    pub struct FormatSpec {
        segments: Vec<Segment>,
    }
    impl FormatSpec {
        pub fn new(segments: Vec<Segment>) -> Self {
            Self {
                segments
            }
        }
        pub fn compile(self) -> FormatScanner {
            todo!()
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum Segment {
        Literal(&'static str),
        Capture(&'static str),
    }
}
