use std::str::FromStr;
use crate::Scan;

impl Scan for i8 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for i16 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for i32 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for i64 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for i128 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for isize {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for u8 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for u16 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for u32 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for u64 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for u128 {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}
impl Scan for usize {
    const REGEX: &'static str = r"-?\d+";
    type Error = <Self as FromStr>::Err;
    fn scan(text: &str, params: &str) -> Result<Self, Self::Error> {
        text.parse()
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn primitive_ints() {
        use super::*;
        assert_eq!(Ok(-4_i8), Scan::scan("-4", ""));
        assert_eq!(Ok(400_i16), Scan::scan("400", ""));
        assert_eq!(Ok(-42_i32), Scan::scan("-42", ""));
        assert_eq!(Ok(429300_i64), Scan::scan("429300", ""));
        assert_eq!(Ok(-950_143_268_751_i128), Scan::scan("-950143268751", ""));
        assert_eq!(Ok(-50_143_268_isize), Scan::scan("-0050143268", ""));
        assert_eq!(Ok(0_u8), Scan::scan("0", ""));
        assert_eq!(Ok(519_u16), Scan::scan("0519", ""));
        assert_eq!(Ok(4_293_000_000_u32), Scan::scan("4293000000", ""));
        assert_eq!(Ok(950_143_268_751_u64), Scan::scan("0000950143268751", ""));
        assert_eq!(Ok(18_446_744_073_709_551_616_u128), Scan::scan("18446744073709551616", ""));
        assert_eq!(Ok(68_751_usize), Scan::scan("000068751", ""));
    }
}
