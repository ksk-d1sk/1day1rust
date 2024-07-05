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

    let k = next!(u8);
    for i in 1..=k {
        let n = next!(usize);
        let mut v = Vec::from_iter((0..n).map(|_| next!(u8)));
        let mut max = 0;

        v.sort_unstable_by(|a, b| b.cmp(a));

        for window in v.windows(2) {
            max = max.max(window[0] - window[1]);
        }

        let _ = writeln!(output, "Class {i}\nMax {}, Min {}, Largest gap {}", v[0], v[n - 1], max);
    }

    print!("{output}");
}