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

    let t = next!(usize);
    for n in (0..t).map(|_| next!(u32)) {
        for i in 0..20 {
            if n & 1 << i != 0 {
                let _ = write!(output, "{i} ");
            }
        }

        output.push('\n');
    }

    print!("{output}");
}