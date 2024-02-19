use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (w1, h1) = next!(i32, i32);
    let (w2, h2) = next!(i32, i32);

    print!("{}", w1 + w2 + h1 * 2 + h2 * 2 + (w1 - w2).abs() + 4);
}