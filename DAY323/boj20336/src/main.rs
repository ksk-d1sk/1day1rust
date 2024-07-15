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

    next!();
    let d = next!(u8);
    let _ = writeln!(output, "{d}");
    for menu in (0..d).map(|_| next!()) {
        let _ = writeln!(output, "{menu}");
    }

    print!("{output}");
}