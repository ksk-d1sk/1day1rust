use std::io::*;
use std::fmt::Write;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u8);
    for k in (0..n).map(|_| next!().as_bytes()) {
        let l = k.len();
        let _ = if k[l - 1] & 1 == 1 {
            writeln!(output, "odd")
        } else {
            writeln!(output, "even")
        };
    }

    print!("{output}");
}