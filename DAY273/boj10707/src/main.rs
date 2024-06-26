use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }


    let (a, b, c, d, p) = next!(u32, u32, u32, u32, u32);

    print!("{}", (p * a).min(b + p.saturating_sub(c) * d));
}