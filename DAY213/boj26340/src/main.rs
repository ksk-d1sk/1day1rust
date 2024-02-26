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

    let n = next!(usize);
    for (a, b, c) in (0..n).map(|_| next!(u32, u32, u32)) {
        let _ = writeln!(output, "Data set: {a} {b} {c}");
        let (mut g, mut l) = if a > b { (a, b) } else { (b, a) };
        for _ in 0..c {
            let d = g / 2;
            (g, l) = if d > l { (d, l) } else { (l, d) };
        }
        let _ = writeln!(output, "{g} {l}\n");
    }

    print!("{output}");
}