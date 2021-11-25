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
