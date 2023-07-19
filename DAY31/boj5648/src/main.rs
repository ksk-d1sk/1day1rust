use std::io::Read;
use std::fmt::Write;

fn main() {
    let mut input  = String::new();
    let mut output = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut v: Vec<u64> = input.split_ascii_whitespace()
        .skip(1)
        .flat_map(|e| String::from_iter(e.chars().rev()).parse())
        .collect();

    v.sort_unstable();

    for e in v {
        let _ = writeln!(output, "{}", e);
    }

    print!("{output}");
}