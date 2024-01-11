use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for b in buf.bytes() {
        output.push(
            match b {
                b'a'..=b'z' => {
                    (b - b'a' + 13) % 26 + b'a'
                },
                b'A'..=b'Z' => {
                    (b - b'A' + 13) % 26 + b'A'
                },
                _ => b,
            } as char
        );
    }

    print!("{output}");
}