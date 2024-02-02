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

    let n = next!(u32);
    for i in 1..=n {
        let _ = match (i % 7, i %11) {
            (0, 0) => writeln!(output, "Wiwat!"),
            (0, _) => writeln!(output, "Hurra!"),
            (_, 0) => writeln!(output, "Super!"),
            (_, _) => writeln!(output, "{i}"),
        };
    }

    print!("{output}");
}