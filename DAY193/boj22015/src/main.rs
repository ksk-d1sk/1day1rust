use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace().flat_map(str::parse::<u8>);

    let mut v = vec![];

    for x in tokens {
        match v.binary_search(&x) {
            Ok(i) => v.insert(i, x),
            Err(i) => v.insert(i, x),
        }
    }

    print!("{}", v[2] * 2 - v[1] - v[0]);
}