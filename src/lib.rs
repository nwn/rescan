mod int;

use regex::Regex;
use rescan_format::format;

pub trait Scan: Sized {
    const REGEX: &'static str;
    type Error: std::error::Error;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error>;
}

#[test]
fn test() {
    assert_eq!("test\\\n", format!(r"test\
"));
    assert_eq!("a  b  {c}", format!("a {test} b {} {{c}}"));
}
