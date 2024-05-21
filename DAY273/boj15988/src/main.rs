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

    let max = 1_000_000;
    let mut dp = vec![0u32, 1, 2, 4];

    dp.reserve(max);

    for i in 4..=max {
        dp.push((dp[i - 1] + dp[i - 2] + dp[i - 3]) % 1_000_000_009);
    }

    let t = next!(usize);
    for n in (0..t).map(|_| next!(usize)) {
        let _ = writeln!(output, "{}", dp[n]);
    }

    print!("{output}");
}