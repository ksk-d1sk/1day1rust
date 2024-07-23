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

    let t = next!(u8);
    for (n, a, d) in (0..t).map(|_| next!(u32, u32, u32)) {
        let mut sum = a * n;
        for i in 0..n {
            sum += d * i;
        }

        let _ = writeln!(output, "{sum}");
    }

    print!("{output}");
}