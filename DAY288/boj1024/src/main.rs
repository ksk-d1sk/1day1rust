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

    let (n, l) = next!(i64, i64);

    for i in l..=100 {
        let m = n / i - (i - 1) / 2;
        if m >= 0 && ((i & 1 == 0 && n * 100 / i % 100 == 50) || (i & 1 == 1 && n % i == 0)) {
            for j in (m..).take(i as usize) {
                let _ = write!(output, "{j} ");
            }
            break;
        }
    }

    if output.is_empty() {
        print!("-1");
    } else {
        print!("{output}");
    }
}