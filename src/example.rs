#![allow(unused)]

use crate::internal::*;

static REGEX_1: LazyRegex = LazyRegex::new(|| {
    Regex::new(r"[[:alpha:]]+\s[[:alpha:]]+")
});
static REGEX_3: LazyRegex = LazyRegex::new(|| {
    Regex::new(r"[[:digit:]]+\s[[:alpha:]]+")
});

fn scanner(reader: &mut impl std::io::BufRead) -> Result<(String, String)> {
    let lit_0 = "One might expect ";
    let lit_2 = " to have at least ";
    let lit_4 = ".";

    let regex_1 = REGEX_1.as_ref()?;
    let regex_3 = REGEX_3.as_ref()?;

    match_literal(reader, lit_0)?;
    let cap_0 = {
        let str = match_regex(reader, regex_1)?;
        let val = <String as std::str::FromStr>::from_str(str).unwrap();
        let str_len = str.len();
        advance_from_regex(reader, str_len);
        val
    };
    match_literal(reader, lit_2)?;
    let cap_1 = {
        let str = match_regex(reader, regex_3)?;
        let val = <String as std::str::FromStr>::from_str(str).unwrap();
        let str_len = str.len();
        advance_from_regex(reader, str_len);
        val
    };
    match_literal(reader, lit_4)?;

    Ok((cap_0, cap_1))
}

#[test]
fn test_scanner() {
    let s = String::from("One might expect most people to have at least 4 fingers.");

    let (sub, obj) = scanner(&mut s.as_bytes()).unwrap();

    assert_eq!("most people", sub);
    assert_eq!("4 fingers", obj);
}
