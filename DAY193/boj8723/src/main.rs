use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace();

    let mut v: Vec<_> = tokens
        .flat_map(|x| x.parse::<u32>().and_then(|n| Ok(n * n)))
        .collect();

    v.sort();

    if v[0] == v[1] && v[1] == v[2] {
        print!("2");
    } else if v[0] + v[1] == v[2] {
        print!("1");
    } else {
        print!("0");
    }
}