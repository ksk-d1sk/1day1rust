use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (a, b, c) = next!(u32, u32, u32);

    if c == 0 {
        print!("{}", int_sqrt(a + b));
    } else {
        print!("{}", c * c - a - b);
    }
}

fn int_sqrt(n: u32) -> u32 {
    (1..=100).collect::<Vec<_>>().partition_point(|x| x * x <= n) as u32
}