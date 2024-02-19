use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c) = next!(u32, u32, u32);
    let (max, min1) = if a > b { (a, b) } else { (b, a) };
    let (max, min2) = if max > c { (max, c) } else { (c, max) };

    if max == min1 + min2 {
        print!("1");
    } else if max == min1 * min2 {
        print!("2");
    } else {
        print!("3");
    }
}