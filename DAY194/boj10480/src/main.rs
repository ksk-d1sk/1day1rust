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
    for x in (0..n).map(|_| next!(i8)) {
        let _ = writeln!(output, "{x} is {}", if x & 1 == 1 { "odd" } else { "even" });
    }

    print!("{output}");
}