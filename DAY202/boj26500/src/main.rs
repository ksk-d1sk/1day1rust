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
    for (a, b) in (0..n).map(|_| next!(f64, f64)) {
        let tmp = (((a - b).abs() * 100.0) as usize + 5) / 10;
        let _ = writeln!(output, "{}.{}", tmp / 10, tmp % 10);
    }

    print!("{output}");
}