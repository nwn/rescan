use rescan;

// Input might look like this: {string: "hello", int: 42, char: 'ß', bool: false, float: 3.14, ignore: _}
fn main() {
    while let Ok((string, float, int, bool, char)) =
        rescan::scanln!("{{string: \"{}\", int: {2}, char: '{4:ch}', bool: {3}, float: {:3}, ignore: {_:ch}}}",
            String, r"-?\d+" as i32, bool, r"-?\d+\.\d+" as f32, ch = char
        )
    {
        println!("{{string: \"{}\", int: {}, char: '{}', bool: {}, float: {}}}", string, int, char, bool, float);
    }
}
