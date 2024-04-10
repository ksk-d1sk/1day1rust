use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
        () => { tokens.next().unwrap() };
    }

    let (n, m, mut k) = next!(u8, u8, u8);
    let whoami = m.min(n / 2);

    if n > m * 2 {
        k = k.saturating_sub(n - m * 2);
    } else {
        k = k.saturating_sub(m - n / 2 + n % 2);
    }

    print!("{}", whoami - (k + 2) / 3);
}