use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
        () => { tokens.next().unwrap() };
    }

    let n = next!(usize);
    let mut dp = vec![[0; 10]; n];

    dp[0] = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1];

    for i in 1..n {
        for j in 0..10 {
            for k in 0..=j {
                dp[i][j] += dp[i - 1][k];
                dp[i][j] %= 10_007;
            }
        }
    }

    print!("{}", dp[n - 1].iter().fold(0_u32, |acc, x| (acc + x) % 10_007));
}