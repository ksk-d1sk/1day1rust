use std::io::*;
use std::fmt::Write;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(usize);
    for i in 1..=t {
        let max = (0..5).map(|_| next!(i32)).max().unwrap();
        let _ = writeln!(output, "Case #{i}: {max}");
    }

    print!("{output}");
}