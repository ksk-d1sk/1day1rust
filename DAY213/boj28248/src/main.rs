use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (p, c) = next!(i32, i32);
    let bonus = if p > c { 500 } else { 0 };

    print!("{}", p * 50 - c * 10 + bonus);
}