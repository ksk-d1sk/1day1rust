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

    for (i, (l, p, v)) in (1_usize..).map(|i| (i, next!(u32, u32, u32))) {
        if l + p + v == 0 {
            break;
        }

        let _ = writeln!(output, "Case {i}: {}", v / p * l + l.min(v % p));
    }

    print!("{output}");
}