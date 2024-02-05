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

    let z = next!(u8);
    for (w, h) in (0..z).map(|_| next!(u32, u32)) {
        let _ = writeln!(output, "{}", w * h / 2);
    }

    print!("{output}");
}