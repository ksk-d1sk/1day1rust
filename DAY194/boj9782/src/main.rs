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

    for i in 1.. {
        let n = next!(usize);

        if n == 0 {
            break;
        }

        let v: Vec<_> = (0..n).map(|_| next!(u32)).collect();
        let mid;

        let _ = if n & 1 == 1 {
            mid = v[n / 2];
            writeln!(
                output,
                "Case {i}: {}.{}",
                mid, 0
            )
        } else {
            mid = (v[n / 2 - 1] + v[n / 2]) * 10 / 2;
            writeln!(
                output,
                "Case {i}: {}.{}",
                mid / 10, mid % 10
            )
        };
    }

    print!("{output}");
}