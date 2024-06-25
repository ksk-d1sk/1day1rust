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
    for (a, b, c, d, e) in (0..t).map(|_| next!(u64, u64, u64, u64, u64)) {
        let mut sum = 0;

        sum += a * 35034;
        sum += b * 23090;
        sum += c * 19055;
        sum += d * 12530;
        sum += e * 18090;

        let _ = writeln!(output, "${}.{:02}", sum / 100, sum % 100);
    }

    print!("{output}");
}