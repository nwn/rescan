use rescan;

// Input might look like this: {string: "hello", int: 42, char: 'ß', bool: false, float: 3.14}
fn main() {
    while let Ok((string, int, char, bool, float)) =
        rescan::scanln!("{{string: \"{}\", int: {}, char: '{}', bool: {}, float: {}}}",
            r"\w+" as String,
            r"-?\d+" as i32,
            "." as char,
            "true|false" as bool,
            r"-?\d+\.\d+" as f32,
        )
    {
        println!("{{string: \"{}\", int: {}, char: '{}', bool: {}, float: {}}}", string, int, char, bool, float);
    }
}
