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
    for _ in 0..n {
        let d = next!(u8);
        let mut check = true;
        let mut prev = next!(u32);

        let _ = write!(output, "Denominations: {prev} ");
        for x in (1..d).map(|_| next!(u32)) {
            let _ = write!(output, "{x} ");
            if x < prev * 2 {
                check = false;
            }
            prev = x;
        }

        let _ = if check {
            writeln!(output, "\nGood coin denominations!\n")
        } else {
            writeln!(output, "\nBad coin denominations!\n")
        };
    }

    print!("{output}");
}