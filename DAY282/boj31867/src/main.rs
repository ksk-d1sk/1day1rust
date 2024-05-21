use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let tokens = input.split_ascii_whitespace();
    let mut even = 0;
    let mut odd  = 0;

    for c in tokens.skip(1).next().unwrap().bytes() {
        if c & 1 == 1 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    match even.cmp(&odd) {
        Greater => print!("1" ),
        Equal   => print!("-1"),
        Less    => print!("0" ),
    }
}