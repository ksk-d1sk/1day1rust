use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (mut x, n) = next!(u32, u16);
    for _ in 0..n {
        if x & 1 == 0 {
            x = (x / 2) ^ 6;
        } else {
            x = (x * 2) ^ 6;
        }
    }

    print!("{x}");
}