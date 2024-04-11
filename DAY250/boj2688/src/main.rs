use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
        () => { tokens.next().unwrap() };
    }

    let size = 64;
    let mut dp = vec![[0; 10]; size];

    dp[0] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    for i in 1..size {
        for j in 0..10 {
            for k in 0..=j {
                dp[i][j] += dp[i - 1][k];
            }
        }
    }

    let t = next!(u16);
    let ans_table: Vec<u64> = dp.iter().map(|v| v.iter().sum()).collect();

    for n in (0..t).map(|_| next!(usize)) {
        let _ = writeln!(output, "{}", ans_table[n - 1]);
    }

    print!("{output}");
}