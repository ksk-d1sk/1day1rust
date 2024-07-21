use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut dp = vec![vec![0; n + 1]; k + 1];

    for i in 1..=k {
        for j in 0..=n {
            dp[i][j] = if j == 0 {
                1
            } else {
                dp[i - 1][j] + dp[i][j - 1]
            } % 1_000_000_000;
        }
    }

    print!("{}", dp[k][n]);
}