use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (p1, q1, p2, q2) = next!(u64, u64, u64, u64);

    print!("{}", ((p1 * p2) % (q1 * q2 * 2) == 0) as u8);
}