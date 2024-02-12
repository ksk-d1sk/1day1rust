use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    for (p, t) in (0..n).map(|_| next!(u32, u32)) {
        let _ = writeln!(
            output,
            "{}",
            p - t / 7 + t / 4
        );
    }

    print!("{output}");
}