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

    let t = next!(u8);
    for (n, m) in (0..t).map(|_| next!(u8, u8)) {
        if n > b'L' - b'A' && m >= 4 {
            let _ = writeln!(output, "{}", m * (b'L' - b'A') + 4);
        } else {
            let _ = writeln!(output, "-1");
        }
    }

    print!("{output}");
}