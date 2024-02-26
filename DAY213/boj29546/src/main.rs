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
    let v: Vec<_> = (0..n).map(|_| next!()).collect();
    let m = next!(u16);

    for (l, r) in (0..m).map(|_| next!(usize, usize)) {
        for i in (l - 1)..r {
            let _ = writeln!(output, "{}", v[i]);
        }
    }

    print!("{output}");
}