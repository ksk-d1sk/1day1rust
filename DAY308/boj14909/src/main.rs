use std::io::*;

fn main() {
    print!("{}", read_to_string(stdin()).unwrap().split_ascii_whitespace().flat_map(str::parse::<i32>).filter(|x| *x > 0).count());
}