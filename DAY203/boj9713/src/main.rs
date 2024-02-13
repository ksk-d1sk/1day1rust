use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let mut v = Vec::from_iter((0..n).map(|_| next!(String, u8)));

    v.sort_unstable_by_key(|(_, level)| *level);

    print!("{}", v[0].0);
}