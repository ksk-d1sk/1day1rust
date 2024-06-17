use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, x) = next!(i16, i16);
    let mut answer = -1;

    for (s, t) in (0..n).map(|_| next!(i16, i16)) {
        if s + t <= x {
            answer = answer.max(s);
        }
    }

    print!("{answer}");
}