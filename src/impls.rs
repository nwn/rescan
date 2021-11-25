use crate::*;
use std::str::FromStr;

macro_rules! impl_scan_as_from_str {
    ($ty:ty) => {
        impl Scan for $ty {
            type Error = <Self as FromStr>::Err;
            fn scan(s: &str) -> Result<Self, Self::Error> {
                <Self as FromStr>::from_str(s)
            }
        }
    }
}

impl_scan_as_from_str!(bool);
impl_scan_as_from_str!(char);
impl_scan_as_from_str!(u8);
impl_scan_as_from_str!(u16);
impl_scan_as_from_str!(u32);
impl_scan_as_from_str!(u64);
impl_scan_as_from_str!(u128);
impl_scan_as_from_str!(usize);
impl_scan_as_from_str!(i8);
impl_scan_as_from_str!(i16);
impl_scan_as_from_str!(i32);
impl_scan_as_from_str!(i64);
impl_scan_as_from_str!(i128);
impl_scan_as_from_str!(isize);
impl_scan_as_from_str!(f32);
impl_scan_as_from_str!(f64);
impl_scan_as_from_str!(std::num::NonZeroU8);
impl_scan_as_from_str!(std::num::NonZeroU16);
impl_scan_as_from_str!(std::num::NonZeroU32);
impl_scan_as_from_str!(std::num::NonZeroU64);
impl_scan_as_from_str!(std::num::NonZeroU128);
impl_scan_as_from_str!(std::num::NonZeroUsize);
impl_scan_as_from_str!(std::num::NonZeroI8);
impl_scan_as_from_str!(std::num::NonZeroI16);
impl_scan_as_from_str!(std::num::NonZeroI32);
impl_scan_as_from_str!(std::num::NonZeroI64);
impl_scan_as_from_str!(std::num::NonZeroI128);
impl_scan_as_from_str!(std::num::NonZeroIsize);
impl_scan_as_from_str!(String);
impl_scan_as_from_str!(std::ffi::OsString);
impl_scan_as_from_str!(std::path::PathBuf);
impl_scan_as_from_str!(std::net::IpAddr);
impl_scan_as_from_str!(std::net::Ipv4Addr);
impl_scan_as_from_str!(std::net::Ipv6Addr);
impl_scan_as_from_str!(std::net::SocketAddr);
impl_scan_as_from_str!(std::net::SocketAddrV4);
impl_scan_as_from_str!(std::net::SocketAddrV6);

macro_rules! impl_default_scan {
    ($ty:ty, $re:expr) => {
        impl DefaultScan for $ty {
            const DEFAULT_REGEX: &'static str = $re;
        }
    }
}

impl_default_scan!(bool, r"true|false");
impl_default_scan!(char, r".");
impl_default_scan!(String, r"\w+");

const UINT_REGEX: &str = r"\+?[0-9]+";
impl_default_scan!(u8, UINT_REGEX);
impl_default_scan!(u16, UINT_REGEX);
impl_default_scan!(u32, UINT_REGEX);
impl_default_scan!(u64, UINT_REGEX);
impl_default_scan!(u128, UINT_REGEX);
impl_default_scan!(usize, UINT_REGEX);
impl_default_scan!(std::num::NonZeroU8, UINT_REGEX);
impl_default_scan!(std::num::NonZeroU16, UINT_REGEX);
impl_default_scan!(std::num::NonZeroU32, UINT_REGEX);
impl_default_scan!(std::num::NonZeroU64, UINT_REGEX);
impl_default_scan!(std::num::NonZeroU128, UINT_REGEX);
impl_default_scan!(std::num::NonZeroUsize, UINT_REGEX);

const INT_REGEX: &str = r"[+-]?[0-9]+";
impl_default_scan!(i8, INT_REGEX);
impl_default_scan!(i16, INT_REGEX);
impl_default_scan!(i32, INT_REGEX);
impl_default_scan!(i64, INT_REGEX);
impl_default_scan!(i128, INT_REGEX);
impl_default_scan!(isize, INT_REGEX);
impl_default_scan!(std::num::NonZeroI8, INT_REGEX);
impl_default_scan!(std::num::NonZeroI16, INT_REGEX);
impl_default_scan!(std::num::NonZeroI32, INT_REGEX);
impl_default_scan!(std::num::NonZeroI64, INT_REGEX);
impl_default_scan!(std::num::NonZeroI128, INT_REGEX);
impl_default_scan!(std::num::NonZeroIsize, INT_REGEX);

// Regex matching the grammar used by `f32`'s and `f64`'s `FromStr` implementation:
// https://doc.rust-lang.org/stable/std/primitive.f32.html#method.from_str
const FLOAT_REGEX: &str = r"[+-]?(?i:inf|nan|(?:[0-9]+\.?[0-9]*|\.[0-9]+)(?:e[+-]?[0-9]+)?)";
impl_default_scan!(f32, FLOAT_REGEX);
impl_default_scan!(f64, FLOAT_REGEX);

// Regex approximately matching IPv4 addresses in their "dotted decimal" notation.
// This pattern is exact except that any 3-digit decimal value is accepted in each
// octet position, rather than the proper range of 0..=255.
const IPV4_REGEX: &str = r"(?:[0-9]{1,3}\.){3}[0-9]{1,3}";
impl_default_scan!(std::net::Ipv4Addr, IPV4_REGEX);

// Regex approximately matching IPv6 addresses according to IETF RFC 4291 Section 2.2.
// This pattern is exact except that:
//  - any 3-digit decimal value is accepted in the octet positions of an embedded IPv4 address
//  - a compressed address (i.e. one containing "::") may consist of more hextet fields than
//    is correct, though still bounded.
// https://datatracker.ietf.org/doc/html/rfc4291#section-2.2
// const IPV6_REGEX: &str = r"(?:[[:xdigit:]]{0,4}:){2,7}(?:[[:xdigit:]]{0,4}|(?:[0-9]{1,3}\.){3}[0-9]{1,3})";
const IPV6_REGEX: &str = r"(?x)
    (?: [[:xdigit:]]{1,4} (?::[[:xdigit:]]{1,4}){7} ) | # Uncompressed
    (?: [[:xdigit:]]{1,4} (?::[[:xdigit:]]{1,4}){5} :[0-9]{1,3} (?:\.[0-9]{1,3}){3} ) | # Uncompressed with embedded IPv4
    (?:
        (?: (?:[[:xdigit:]]{1,4}:)+ | : )
        (?:
            (?: : (?:(?:[0-9]{1,3}\.){3}[0-9]{1,3})? ) | # Compressed at end (with optional embedded IPv4)
            (?: (?::[[:xdigit:]]{1,4})+ (?::(?:[0-9]{1,3}\.){3}[0-9]{1,3})? ) # Compressed at start or middle (with optional embedded IPv4)
        )
    )";
impl_default_scan!(std::net::Ipv6Addr, IPV6_REGEX);

// Regex approximately matching both IPv4 and IPv6 addresses. This pattern is the union of the
// [`IPV4_REGEX`] and [`IPV6_REGEX`] patterns, and the same caveats apply.
const IP_REGEX: &str = r"(?x)
    (?: (?:[0-9]{1,3}\.){3}[0-9]{1,3} ) | # IPv4
    (?: [[:xdigit:]]{1,4} (?::[[:xdigit:]]{1,4}){7} ) | # Uncompressed IPv6
    (?: [[:xdigit:]]{1,4} (?::[[:xdigit:]]{1,4}){5} :[0-9]{1,3} (?:\.[0-9]{1,3}){3} ) | # Uncompressed IPv6 with embedded IPv4
    (?:
        (?: (?:[[:xdigit:]]{1,4}:)+ | : )
        (?:
            (?: : (?:(?:[0-9]{1,3}\.){3}[0-9]{1,3})? ) | # IPv6 compressed at end (with optional embedded IPv4)
            (?: (?::[[:xdigit:]]{1,4})+ (?::(?:[0-9]{1,3}\.){3}[0-9]{1,3})? ) # IPv6 compressed at start or middle (with optional embedded IPv4)
        )
    )";
impl_default_scan!(std::net::IpAddr, IP_REGEX);

#[cfg(test)]
mod test {
    use crate::DefaultScan;

    fn default_regex<T: DefaultScan>() -> regex::Regex {
        regex::Regex::new(&format!(r"\A(?:{})\z", T::DEFAULT_REGEX)).unwrap()
    }

    #[test]
    fn ipv4() {
        let re = default_regex::<std::net::Ipv4Addr>();
        assert!(!re.is_match(""));
        assert!(!re.is_match("0.0.0"));
        assert!(!re.is_match(".0.0.0"));
        assert!(!re.is_match("0..0.0"));
        assert!(!re.is_match("0.0..0"));
        assert!(!re.is_match("0.0.0."));
        assert!(!re.is_match("0.0.0.0."));
        assert!(!re.is_match("0.0.0.0.0"));
        assert!(!re.is_match("1000.0.0.0"));
        assert!(!re.is_match("0.0.0.1000"));

        assert!(re.is_match("0.0.0.0"));
        assert!(re.is_match("255.0.0.0"));
        assert!(re.is_match("0.255.0.0"));
        assert!(re.is_match("0.0.255.0"));
        assert!(re.is_match("0.0.0.255"));
        assert!(re.is_match("255.255.255.255"));
        assert!(re.is_match("10.27.0.255"));
        assert!(re.is_match("192.168.0.1"));
        assert!(re.is_match("010.0.00.001"));
    }

    #[test]
    fn ipv6() {
        let re = default_regex::<std::net::Ipv6Addr>();
        assert!(!re.is_match(""));
        assert!(!re.is_match(":"));
        assert!(!re.is_match(":1"));
        assert!(!re.is_match("192.168.0.1"));
        assert!(!re.is_match(":192.168.0.1"));
        assert!(!re.is_match("::10.0.0.0.1"));
        assert!(!re.is_match("::1000.0.0.0"));
        assert!(!re.is_match(":ff01::101"));
        assert!(!re.is_match("ff01::101:"));

        assert!(re.is_match("::"));
        assert!(re.is_match("::1"));
        assert!(re.is_match("::192.168.0.1"));
        assert!(re.is_match("::1:192.168.0.1"));
        assert!(re.is_match("abcd:ef01:2345:6789:abcd:ef01:2345:6789"));
        assert!(re.is_match("ABCD:EF01:2345:6789:ABCD:EF01:2345:6789"));
        assert!(re.is_match("abcd:ef01:2345:6789:abcd:ef01:192.168.0.1"));
        assert!(re.is_match("2001:db8:0:0:8:800:200c:417a"));
        assert!(re.is_match("2001:db8::8:800:200c:417a"));
        assert!(re.is_match("ff01::101"));
        assert!(re.is_match("ff01::101:192.168.0.1"));
    }

    #[test]
    fn ip() {
        let re = default_regex::<std::net::IpAddr>();
        assert!(!re.is_match(""));
        assert!(!re.is_match("0.0.0"));
        assert!(!re.is_match(".0.0.0"));
        assert!(!re.is_match("0..0.0"));
        assert!(!re.is_match("0.0..0"));
        assert!(!re.is_match("0.0.0."));
        assert!(!re.is_match("0.0.0.0."));
        assert!(!re.is_match("0.0.0.0.0"));
        assert!(!re.is_match("1000.0.0.0"));
        assert!(!re.is_match("0.0.0.1000"));

        assert!(re.is_match("0.0.0.0"));
        assert!(re.is_match("255.0.0.0"));
        assert!(re.is_match("0.255.0.0"));
        assert!(re.is_match("0.0.255.0"));
        assert!(re.is_match("0.0.0.255"));
        assert!(re.is_match("255.255.255.255"));
        assert!(re.is_match("10.27.0.255"));
        assert!(re.is_match("192.168.0.1"));
        assert!(re.is_match("010.0.00.001"));

        assert!(!re.is_match(":"));
        assert!(!re.is_match(":1"));
        assert!(!re.is_match(":192.168.0.1"));
        assert!(!re.is_match("::10.0.0.0.1"));
        assert!(!re.is_match("::1000.0.0.0"));
        assert!(!re.is_match(":ff01::101"));
        assert!(!re.is_match("ff01::101:"));

        assert!(re.is_match("::"));
        assert!(re.is_match("::1"));
        assert!(re.is_match("::192.168.0.1"));
        assert!(re.is_match("::1:192.168.0.1"));
        assert!(re.is_match("abcd:ef01:2345:6789:abcd:ef01:2345:6789"));
        assert!(re.is_match("ABCD:EF01:2345:6789:ABCD:EF01:2345:6789"));
        assert!(re.is_match("abcd:ef01:2345:6789:abcd:ef01:192.168.0.1"));
        assert!(re.is_match("2001:db8:0:0:8:800:200c:417a"));
        assert!(re.is_match("2001:db8::8:800:200c:417a"));
        assert!(re.is_match("ff01::101"));
        assert!(re.is_match("ff01::101:192.168.0.1"));
    }
}
