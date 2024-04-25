use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    let (mut a, mut b) = (100, 100);

    for (c, d) in (0..n).map(|_| next!(u8, u8)) {
        if c > d {
            b -= c;
        } else if c < d {
            a -= d;
        }
    }

    print!("{a}\n{b}");
}