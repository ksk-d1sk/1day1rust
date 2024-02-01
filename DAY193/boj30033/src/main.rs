use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);

    print!(
        "{}",
        (0..n)
            .map(|_| next!(u8))
            .collect::<Vec<_>>()
            .into_iter()
            .zip((0..n).map(|_| next!(u8)))
            .filter(|(a, b)| a <= b)
            .count()
    );
}