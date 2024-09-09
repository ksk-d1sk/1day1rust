use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = vec![vec![0; 3]; n + 1];

    dp[1][0] = 1;
    dp[1][1] = 1;
    dp[1][2] = 1;

    for i in 2..=n {
        dp[i][0] += dp[i - 1][0];
        dp[i][0] += dp[i - 1][1];
        dp[i][0] += dp[i - 1][2];
        dp[i][0] %= 9901;
        
        dp[i][1] += dp[i - 1][0];
        dp[i][1] += dp[i - 1][2];
        dp[i][1] %= 9901;
        
        dp[i][2] += dp[i - 1][0];
        dp[i][2] += dp[i - 1][1];
        dp[i][2] %= 9901;
    }

    print!("{}", dp[n].iter().sum::<u32>() % 9901);
}