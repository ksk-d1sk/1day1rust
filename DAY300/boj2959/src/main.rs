use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut v = Vec::from_iter((0..4).map(|_| next!(u32)));

    v.sort_unstable();

    print!("{}", v[0] * v[2]);
}