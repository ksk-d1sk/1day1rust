use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c) = next!(u16, u16, u16);

    if a - b - c == 0 ||
       b - a - c == 0 ||
       c - a - b == 0
    {
        print!("1");
    } else {
        print!("0");
    }
}