use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);
    let iter: Vec<_> = (0..n).map(|_| next!()).collect();
    let c = next!();

    print!("{}", iter.into_iter().filter(|s| *s == c).count())
}