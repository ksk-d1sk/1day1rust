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
    for (x, y) in (0..t).map(|_| next!(usize, usize)) {
        let _ = if x >= y {
            writeln!(output, "MMM BRAINS")
        } else {
            writeln!(output, "NO BRAINS")
        };
    }

    print!("{output}");
}