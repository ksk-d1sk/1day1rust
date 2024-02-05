use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut v: Vec<i16> = input.split_ascii_whitespace().flat_map(str::parse).collect();

    v.sort();

    if v[2] == v[1] || v[2] == v[1] + v[0] || v[1] == v[0] {
        print!("S");
    } else {
        print!("N");
    }
}