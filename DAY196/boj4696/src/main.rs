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

    for n in (0..).map(|_| next!(f64)).take_while(|&n| n != 0.0) {
        let temp = 1.0 + n + n * n + n * n * n + n * n * n * n;
        let _ = writeln!(output, "{:.2}", temp);
    }

    print!("{output}");
}