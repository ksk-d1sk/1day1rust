use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, a, b, c, d) = next!(u16, u16, u16, u16, u16);

    print!("{}", ((n + a - 1) / a * b).min((n + c - 1) / c * d));
}