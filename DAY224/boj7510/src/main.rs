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
    for i in 1..=n {
        let (a, b, c) = next!(u32, u32, u32);
        let (max, next1) = if a > b { (a, b) } else { (b, a) };
        let (max, next2) = if max > c { (max, c) } else { (c, max) };

        let _ = writeln!(output, "Scenario #{i}:");
        let _ = if max * max == next1 * next1 + next2 * next2 {
            writeln!(output, "yes\n")
        } else {
            writeln!(output, "no\n")
        };
    }

    print!("{output}");
}