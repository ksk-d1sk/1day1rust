use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (mut x, mut y) = next!(f64, f64);
    let mut min = 1000.0 / y * x;

    let n = next!(u8);
    for _ in 0..n {
        (x, y) = next!(f64, f64);
        min = min.min(1000.0 / y * x);
    }

    print!("{min}");
}