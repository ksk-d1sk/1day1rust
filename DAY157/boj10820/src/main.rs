use std::fmt::Write;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut output = String::new();

    for line in input.lines() {
        let mut lower = 0;
        let mut upper = 0;
        let mut number = 0;
        let mut whitespace = 0;

        for b in line.bytes() {
            match b {
                b'a'..=b'z' => lower += 1,
                b'A'..=b'Z' => upper += 1,
                b'0'..=b'9' => number += 1,
                _           => whitespace += 1,
            }
        }

        let _ = writeln!(
            output,
            "{} {} {} {}",
            lower,
            upper,
            number,
            whitespace
        );
    }

    print!("{output}");
}