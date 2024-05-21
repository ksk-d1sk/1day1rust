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
    for (n, m) in (0..t).map(|_| next!(u32, u32)) {
        let mut count = 0;
        for a in 2..n {
            for b in 1..a {
                if (a * a + b * b + m) % (a * b) == 0 {
                    count += 1;
                }
            }
        }
        let _ = writeln!(output, "{count}");
    }

    print!("{output}");
}