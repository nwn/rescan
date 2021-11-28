use rescan_macros::scanner;

#[test]
fn test() {
    let _scanner: rescan::Scanner<(String, String)> = scanner!("One might expect {} to have at least {}.",
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

    let (subject, object) = scanner.scan(reader).unwrap();
    assert_eq!("most people", subject);
    assert_eq!("4 fingers", object);
}


#[test]
fn line_buf() {
    let string = concat!(
        "One might expect most people to have at least 4 fingers.\n",
        "One might expect few pirates to have at least 2 eyes.",
    );
    let reader = &mut string.as_bytes();

    let scanner = scanner!("One might expect {} to have at least {}.",
        r"[[:alpha:]]+\s[[:alpha:]]+" as String,
        r"[[:digit:]]+\s[[:alpha:]]+" as String,
    );

    let (subject, object) = scanner.scan_lines(reader).next().unwrap().unwrap();
    assert_eq!("most people", subject);
    assert_eq!("4 fingers", object);
    let (subject, object) = scanner.scan_lines(reader).next().unwrap().unwrap();
    assert_eq!("few pirates", subject);
    assert_eq!("2 eyes", object);

    assert!(scanner.scan_lines(reader).next().is_none());
}


#[test]
fn line_iter() {
    let string = concat!(
        "One might expect most people to have at least 4 fingers.\n",
        "One might expect few pirates to have at least 2 eyes.",
    );
    let reader = &mut string.as_bytes();

    let scanner = scanner!("One might expect {} to have at least {}.",
        r"[[:alpha:]]+\s[[:alpha:]]+" as String,
        r"[[:digit:]]+\s[[:alpha:]]+" as String,
    );

    let result: Vec<_> = scanner.scan_lines(reader).map(Result::unwrap).collect();
    assert_eq!(result, vec![
        ("most people".into(), "4 fingers".into()),
        ("few pirates".into(), "2 eyes".into()),
    ]);
}
