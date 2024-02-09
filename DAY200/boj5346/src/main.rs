use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    for n in (0..).map(|_| next!(u32)).take_while(|n| *n != 0) {
        let _ = writeln!(output, "{}", (n + 1) / 2);
    }

    print!("{output}");
}