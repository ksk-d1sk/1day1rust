use std::io::*;
use std::fmt::Write;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().trim().parse::<$t>().unwrap()),+) };
    }

    let (n, d) = next!(u8, u32);
    let v: Vec<_> = (0..n).map(|_| next!(u32)).collect();
    let sum: u32 = v.iter().sum();

    for x in v {
        let _  = writeln!(output, "{}", d * x / sum);
    }

    print!("{output}");
}