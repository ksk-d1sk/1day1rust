use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for (a, b) in (0..t).map(|_| next!(i32, i32)) {
        let (mut max, mut min) = if a > b { (a, b) } else { (b, a) };
        let mut m = max % min;

        while m != 0 {
            (max, min) = (min, m);
            m = max % min;
        }

        let _ = writeln!(output, "{min}");
    }

    print!("{output}");
}