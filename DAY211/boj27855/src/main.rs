use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (h1, b1) = next!(u8, u8);
    let (h2, b2) = next!(u8, u8);

    let score_1 = h1 * 3 + b1;
    let score_2 = h2 * 3 + b2;

    match score_1.cmp(&score_2) {
        Greater => print!("1 {}", score_1 - score_2),
        Equal => print!("NO SCORE"),
        Less => print!("2 {}", score_2 - score_1),
    }
}