use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u16);
    for n in (0..t).map(|_| next!(u64)) {
        let _ = writeln!(output, "{}", pow(2, n.saturating_sub(2)));
    }

    print!("{output}");
}

fn pow(mut a: u64, mut b: u64) -> u64 {
    let mut ret = 1;
    while b != 0 {
        if b & 1 == 1 {
            ret = ret * a % 1_000_000_007;
        }
        a = a * a % 1_000_000_007;
        b >>= 1;
    }

    ret
}