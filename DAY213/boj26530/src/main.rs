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
    for _ in 0..n {
        let x = next!(usize);
        let mut sum = 0.0;
        for _ in 0..x {
            let _ = next!();
            let a = next!(f64);
            let b = next!(f64);
            sum += a * b;
        }
        let _ = writeln!(output, "${:.02}", sum);
    }

    print!("{output}");
}