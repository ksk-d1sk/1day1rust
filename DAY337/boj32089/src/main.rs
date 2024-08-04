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

    (0..)
        .map(|_| {
            let n = next!(u16);
            Vec::from_iter((0..n).map(|_| next!(u32)))
        })
        .take_while(|v| !v.is_empty())
        .for_each(|v| {
            let answer = v.windows(3).map(|window| window.iter().sum::<u32>()).max().unwrap();
            let _ = writeln!(output, "{answer}");
        });

    print!("{output}");
}