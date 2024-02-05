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
    for i in 1..=n {
        let _ = write!(output, "{i} ");

        if i % 6 == 0 {
            output.push_str("Go! ");
        }
    }

    if n % 6 != 0 {
        output.push_str("Go!");
    }

    print!("{output}");
}