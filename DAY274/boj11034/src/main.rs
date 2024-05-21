use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut output = String::new();

    for line in input.trim().lines() {
        let mut tokens = line.split_ascii_whitespace();
        let a: u8 = tokens.next().unwrap().parse().unwrap();
        let b: u8 = tokens.next().unwrap().parse().unwrap();
        let c: u8 = tokens.next().unwrap().parse().unwrap();
        let _ = writeln!(output, "{}", (b - a - 1).max(c - b - 1));
    }

    print!("{output}");
}