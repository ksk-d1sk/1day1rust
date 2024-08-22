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
    let ceil = Vec::from_iter((0..n).map(|_| next!(i16)));
    let floor = Vec::from_iter((0..n).map(|_| next!(i16)));
    let mut x = 0;

    for i in 0..n {
        if x >= ceil[i] {
            x -= x - ceil[i] + 1;
        } else if x <= floor[i] {
            x += floor[i] - x + 1;
        }

        let _ = write!(output, "{x} ");
    }

    print!("{output}");
}