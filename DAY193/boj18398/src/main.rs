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
    for _ in 0..t {
        let n = next!(usize);

        for (a, b) in (0..n).map(|_| next!(i32, i32)) {
            let _ = writeln!(output, "{} {}", a + b, a * b);
        }
    }

    print!("{output}");
}