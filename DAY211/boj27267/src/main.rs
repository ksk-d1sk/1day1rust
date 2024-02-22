use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (a, b) = next!(u32, u32);
    let (c, d) = next!(u32, u32);

    match (a * b).cmp(&(c * d)) {
        Greater => print!("M"),
        Equal => print!("E"),
        Less => print!("P"),
    }
}