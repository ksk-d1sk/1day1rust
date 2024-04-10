use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = vec![[0; 10]; n];

    dp[0] = [0, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    for i in 1..n {
        for j in 0..=9 {
            if j != 0 {
                dp[i][j - 1] += dp[i - 1][j];
                dp[i][j - 1] %= 1_000_000_000;
            }

            if j != 9 {
                dp[i][j + 1] += dp[i - 1][j];
                dp[i][j + 1] %= 1_000_000_000;
            }
        }
    }

    print!("{}", dp[n - 1].iter().fold(0, |acc, x| (acc + x) % 1_000_000_000));
}