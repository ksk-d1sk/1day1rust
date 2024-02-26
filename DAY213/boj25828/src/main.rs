use std::cmp::Ordering::*;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (g, p, t) = next!(u32, u32, u32);

    match (g + t * p).cmp(&(g * p)) {
        Equal => print!("0"),
        Greater => print!("1"),
        Less => print!("2"),
    }
}