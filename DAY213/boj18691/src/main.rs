use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u8);
    for (g, c, e) in (0..t).map(|_| next!(u8, u16, u16)) {
        let a = e.saturating_sub(c);
        let _ = if g == 1 {
            writeln!(output, "{}", a)
        } else if g == 2 {
            writeln!(output, "{}", a * 3)
        } else {
            writeln!(output, "{}", a * 5)
        };
    }

    print!("{output}");
}