#![allow(unused)]

use crate::internal::*;

fn build_regexes() -> Result<Vec<Regex>, RegexError> {
    [
        Regex::new(r"[[:alpha:]]+\s[[:alpha:]]+"),
        Regex::new(r"[[:digit:]]+\s[[:alpha:]]+"),
    ].into_iter().collect()
}

fn scan(reader: &mut dyn std::io::BufRead, regexes: &[Regex]) -> Result<(String, String)> {
    use crate::{Scan, DefaultScan, Error};

    let lit_0 = "One might expect ";
    let lit_1 = " to have at least ";
    let lit_2 = ".";

    match_literal(reader, lit_0)?;
    let cap_0 = {
        let str = match_regex(reader, &regexes[0])?;
        let val = <String as Scan>::scan(str).map_err(Error::from_parse_error)?;
        let str_len = str.len();
        advance_from_regex(reader, str_len);
        val
    };
    match_literal(reader, lit_1)?;
    let cap_1 = {
        let str = match_regex(reader, &regexes[1])?;
        let val = <String as Scan>::scan(str).map_err(Error::from_parse_error)?;
        let str_len = str.len();
        advance_from_regex(reader, str_len);
        val
    };
    match_literal(reader, lit_2)?;

    Ok((cap_0, cap_1))
}

#[test]
fn test_scanner() {
    let s = String::from("One might expect most people to have at least 4 fingers.");

    let scanner = Scanner::new(build_regexes, scan);
    let (sub, obj) = scanner.scan(&mut s.as_bytes()).unwrap();

    assert_eq!("most people", sub);
    assert_eq!("4 fingers", obj);
}
