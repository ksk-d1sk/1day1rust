use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c) = next!(u8, u8, u8);

    if a != b && a != c {
        print!("A");
    } else
    if b != a && b != c {
        print!("B");
    } else
    if c != a && c != b {
        print!("C");
    } else
    {
        print!("*");
    }
}