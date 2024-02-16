use std::fmt::Write;
use std::io;

fn main() {
    let input = io::read_to_string(io::stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (x, y) = next!(u32, u32);
    for a in (0..).map(|a| x + a * 60).take_while(|&a| a <= y) {
        let _ = writeln!(output, "All positions change in year {a}");
    }

    print!("{output}");
}