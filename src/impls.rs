use crate::*;
use std::{marker::PhantomData, str::FromStr};

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

/// Implements [`Scan`](crate::Scan) for integer types, interpreting input
/// as binary.
///
/// This type can be extended to support custom types.
pub struct Binary<T> { _phantom: PhantomData<T> }
impl<T> Binary<T> {
    const RADIX: u32 = 2;
    const UINT_REGEX: &'static str = r"\+?[01]+";
    const INT_REGEX: &'static str = r"[+-]?[01]+";
}

/// Implements [`Scan`](crate::Scan) for integer types, interpreting input
/// as octal.
///
/// This type can be extended to support custom types.
pub struct Octal<T> { _phantom: PhantomData<T> }
impl<T> Octal<T> {
    const RADIX: u32 = 8;
    const UINT_REGEX: &'static str = r"\+?[0-7]+";
    const INT_REGEX: &'static str = r"[+-]?[0-7]+";
}

/// Implements [`Scan`](crate::Scan) for integer types, interpreting input
/// as hexadecimal.
///
/// This type can be extended to support custom types.
pub struct Hex<T> { _phantom: PhantomData<T> }
impl<T> Hex<T> {
    const RADIX: u32 = 16;
    const UINT_REGEX: &'static str = r"\+?[0-9A-Za-z]+";
    const INT_REGEX: &'static str = r"[+-]?[0-9A-Za-z]+";
}

macro_rules! impl_scan_as_from_str_radix_ {
    ($adaptor:ident, $output:ty) => {
        impl Scan for $adaptor<$output> {
            type Output = $output;
            type Error = std::num::ParseIntError;
            fn scan(s: &str) -> Result<Self::Output, Self::Error> {
                Self::Output::from_str_radix(s, Self::RADIX)
            }
        }
        impl DefaultScan for $adaptor<$output> {
            const DEFAULT_REGEX: &'static str = if Self::Output::MIN == 0 {
                Self::UINT_REGEX
            } else {
                Self::INT_REGEX
            };
        }
    }
}

macro_rules! impl_scan_as_from_str_radix {
    ($($output:ty),*) => {$(
        impl_scan_as_from_str_radix_!(Binary, $output);
        impl_scan_as_from_str_radix_!(Octal, $output);
        impl_scan_as_from_str_radix_!(Hex, $output);
    )+}
}

impl_scan_as_from_str_radix!(u8, u16, u32, u64, u128, usize);
impl_scan_as_from_str_radix!(i8, i16, i32, i64, i128, isize);
