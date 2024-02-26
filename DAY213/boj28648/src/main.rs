use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let mut min = u8::MAX;

    for (t, l) in (0..n).map(|_| next!(u8, u8)) {
        min = min.min(t + l);
    }

    print!("{min}");
}