use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for (i, n0) in (1..).map(|i| (i, next!(u32))).take_while(|(_, n0)| *n0 != 0) {
        let _ = writeln!(
            output,
            "{i}. {} {}",
            if n0 & 1 == 1 { "odd" } else { "even" },
            n0 / 2
        );
    }

    print!("{output}");
}