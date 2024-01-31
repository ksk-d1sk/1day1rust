use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();

    let a: u32 = input.split_ascii_whitespace()
        .skip(1)
        .flat_map(str::parse)
        .map(|x| [x])
        .collect::<Vec<[_; 1]>>()
        .join(&8)
        .iter()
        .sum();

    print!("{} {}", a / 24, a % 24);
}