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


#[test]
fn line_buf() {
    let string_1 = "One might expect most people to have at least 4 fingers.";
    let string_2 = "One might expect few pirates to have at least 2 eyes.";
    let string = format!("{}\n{}", string_1, string_2);
    let reader = &mut string.as_bytes();
    let line_reader = &mut rescan::readers::LineReader::new(reader);

    let scanner = scanner!("One might expect {} to have at least {}.",
        r"[[:alpha:]]+\s[[:alpha:]]+" as String,
        r"[[:digit:]]+\s[[:alpha:]]+" as String,
    );

    let (subject, object) = scanner(line_reader).unwrap();
    assert_eq!("most people", subject);
    assert_eq!("4 fingers", object);
    let (subject, object) = scanner(line_reader).unwrap();
    assert_eq!("few pirates", subject);
    assert_eq!("2 eyes", object);

    assert_eq!(&[0u8;0], std::io::BufRead::fill_buf(line_reader).unwrap());
}
