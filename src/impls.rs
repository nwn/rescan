use crate::*;
use std::str::FromStr;

macro_rules! impl_scan_as_from_str {
    ($ty:ty) => {
        impl Scan for $ty {
            type Output = Self;
            type Error = <Self as FromStr>::Err;
            fn scan(s: &str) -> Result<Self::Output, Self::Error> {
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

impl_default_scan!(bool, "true|false");
impl_default_scan!(char, ".");
impl_default_scan!(String, r"\w+");

impl_default_scan!(u8, "[[:digit:]]+");
impl_default_scan!(u16, "[[:digit:]]+");
impl_default_scan!(u32, "[[:digit:]]+");
impl_default_scan!(u64, "[[:digit:]]+");
impl_default_scan!(u128, "[[:digit:]]+");
impl_default_scan!(usize, "[[:digit:]]+");

impl_default_scan!(i8, "-?[[:digit:]]+");
impl_default_scan!(i16, "-?[[:digit:]]+");
impl_default_scan!(i32, "-?[[:digit:]]+");
impl_default_scan!(i64, "-?[[:digit:]]+");
impl_default_scan!(i128, "-?[[:digit:]]+");
impl_default_scan!(isize, "-?[[:digit:]]+");
