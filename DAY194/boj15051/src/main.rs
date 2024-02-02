use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c) = next!(u32, u32, u32);
    let mut min = u32::MAX;

    min = min.min(a * 2 + c * 2);
    min = min.min(b * 2 + c * 4);
    min = min.min(a * 4 + b * 2);

    print!("{min}");
}