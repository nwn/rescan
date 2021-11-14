// Input might look like this: {string: "hello", int: 42, char: 'ß', bool: false, float: 3.14, ignore: _}
fn main() {
    let stdin = std::io::stdin();
    let line_scanner = rescan::scan_lines!(stdin.lock(), "{}", ".*" as String);

    line_scanner.map(Result::unwrap).for_each(|(str,)| println!("{}", str));
}
