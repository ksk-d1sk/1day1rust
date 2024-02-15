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
    for (id, a, b, c) in (0..n).map(|_| next!(u32, u16, u16, u16)) {
        let mut result = "FAIL";
        let total_score = a + b + c;
        if (a >= 11) && (b >= 8) && (c >= 12) && total_score >= 55 {
            result = "PASS";
        }

        let _ = writeln!(output, "{id} {total_score} {result}");
    }

    print!("{output}");
}