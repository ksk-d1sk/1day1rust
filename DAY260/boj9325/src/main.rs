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

    let t = next!(usize);
    for _ in 0..t {
        let mut payment = next!(u32);
        let n = next!(u16);

        for (q, p) in (0..n).map(|_| next!(u32, u32)) {
            payment += p * q;
        }

        let _ = writeln!(output, "{payment}");
    }

    print!("{output}");
}