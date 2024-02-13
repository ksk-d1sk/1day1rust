use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = next!(u8);
    for n in (0..t).map(|_| next!(u8)) {
        let _ = write!(output, "Pairs for {n}:");
        for i in (1..).take_while(|&i| i < n - i) {
            if i > 1 {
                output.push(',');
            }

            let _ = write!(output, " {} {}", i, n - i);
        }

        output.push('\n');
    }

    print!("{output}");
}