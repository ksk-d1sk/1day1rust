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
        let n = next!(u32);
        let (sum_g, sum_c) = (0..n).map(|_| next!(u32, f64)).fold((0, 0.0), |acc, (c, g)| {
            (acc.0 + c, acc.1 + g * c as f64)
        });

        let _ = writeln!(output, "{sum_g} {}", sum_c / sum_g as f64);
    }

    print!("{output}");
}