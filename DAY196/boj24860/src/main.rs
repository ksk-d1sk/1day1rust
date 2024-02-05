use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (v1, j1) = next!(u64, u64);
    let (v2, j2) = next!(u64, u64);
    let (v3, d3, j3) = next!(u64, u64, u64);

    print!("{}", (v1 * j1 + v2 * j2) * v3 * d3 * j3);
}