use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let input = buf.split_ascii_whitespace();
    let mut output = String::new();

    let mut v: Vec<u32> = input
        .skip(2)
        .flat_map(str::parse)
        .collect();

    v.sort_unstable();

    for e in v.iter() {
        let _ = write!(output, "{e} ");
    }

    print!("{output}");
}
