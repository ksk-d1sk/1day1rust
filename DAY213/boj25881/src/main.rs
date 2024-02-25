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

    let (a, b) = next!(u32, u32);
    let n = next!(usize);

    for x in (0..n).map(|_| next!(u32)) {
        let _  = writeln!(output, "{x} {}", x.min(1000) * a + x.saturating_sub(1000) * b);
    }

    print!("{output}");
}