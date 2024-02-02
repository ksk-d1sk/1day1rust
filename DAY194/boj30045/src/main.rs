use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);
    let mut count = 0;

    for s in (0..n).map(|_| next!()) {
        count += (s.contains("01") || s.contains("OI")) as u16
    }

    print!("{count}");
}