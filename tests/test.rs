use rescan_macros::scanner;

#[test]
fn test() {
    let scanner: rescan::Scanner<std::io::BufReader<std::io::Stdin>, (String, String)> = scanner!("One might expect {} to have at least {}.",
        r"[[:alpha:]]+\s[[:alpha:]]+" as String,
        r"[[:digit:]]+\s[[:alpha:]]+" as String,
    );
}


#[test]
fn string_buf() {
    let reader = &mut "One might expect most people to have at least 4 fingers.".as_bytes();
    let scanner = scanner!("One might expect {} to have at least {}.",
        r"[[:alpha:]]+\s[[:alpha:]]+" as String,
        r"[[:digit:]]+\s[[:alpha:]]+" as String,
    );

    let (subject, object) = scanner(reader).unwrap();
    assert_eq!("most people", subject);
    assert_eq!("4 fingers", object);
}
