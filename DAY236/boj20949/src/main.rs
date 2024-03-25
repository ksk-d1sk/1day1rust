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

    let n = next!(u16);

    let mut v: Vec<_> = (1..=n)
        .map(|i| {
            let (a, b) = next!(u32, u32);
            (i , a.pow(2) + b.pow(2))
        })
        .collect();

    v.sort_by(|l, r| r.1.cmp(&l.1));

    for (i, _) in v {
        let _ = writeln!(output, "{i}");
    }

    print!("{output}");
}