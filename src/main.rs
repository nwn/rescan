// Input might look like this: {string: "hello", int: 42, char: 'ß', bool: false, float: 3.14, ignore: _}
fn main() {
    println!("{}", rescan::scanln!("{}", ".*" as String).unwrap().0);

    //let stdin = std::io::stdin();
    //let scanner = rescan::scanner!("{}", ".*" as String);

    //scanner.scan_lines(&mut stdin.lock()).map(Result::unwrap).for_each(|(str,)| println!("{}", str));
}
