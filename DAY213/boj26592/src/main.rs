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

    let n = next!(usize);
    for (a, b) in (0..n).map(|_| next!(f64, f64)) {
        let _ = writeln!(output, "The height of the triangle is {:.02} units", a * 2.0 / b);
    }

    print!("{output}");
}