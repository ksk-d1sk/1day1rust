use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    next!();
    let mut a = 0;
    let mut b = 0;

    for c in next!().bytes() {
        if c == b'A' {
            a += 1;
        } else {
            b += 1;
        }
    }

    match a.cmp(&b) {
        Greater => print!("A"),
        Equal => print!("Tie"),
        Less => print!("B"),
    }
}