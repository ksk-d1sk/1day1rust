use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace().flat_map(str::parse);

    let v: Vec<_> = tokens.collect();
    let mut min = u16::MAX;

    for x in v.iter() {
        min = min.min(*x);
    }

    print!("{}", v.iter().sum::<u16>() - min + 1);
}