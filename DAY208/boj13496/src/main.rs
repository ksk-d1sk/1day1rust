use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let k = next!(usize);
    for i in 1..=k {
        let (n, s, d) = next!(u8, u32, u32);
        let mut ans = 0;
        let temp = s * d;
        for (d, v) in (0..n).map(|_| next!(u32, u32)) {
            if d <= temp {
                ans += v;
            }
        }

        let _ = writeln!(output, "Data Set {i}:\n{ans}\n");
    }

    print!("{output}");
}