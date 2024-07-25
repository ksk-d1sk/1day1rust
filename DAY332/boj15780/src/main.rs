use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (mut n, k) = next!(i16, u8);

    for a in (0..k).map(|_| next!(i16)) {
        n -= (a + 1) / 2;
    }

    if n > 0 {
        print!("NO");
    } else {
        print!("YES");
    }
}