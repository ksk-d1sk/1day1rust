use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (c4, a3, a4) = next!(f64, f64, f64);

    print!("{}", (c4 * 229.0 * 324.0 * 2.0 + a3 * 297.0 * 420.0 * 2.0 + a4 * 210.0 * 297.0) * 0.000001);
}