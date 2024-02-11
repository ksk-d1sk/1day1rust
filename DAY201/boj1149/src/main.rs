use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(u16);
    let mut ans = [0, 0, 0];
    let mut dp = [[0; 3]; 2];

    for (r, g, b) in (0..n).map(|_| next!(u32, u32, u32)) {
        dp[0][0] = ans[1] + r;
        dp[1][0] = ans[2] + r;

        dp[0][1] = ans[0] + g;
        dp[1][1] = ans[2] + g;

        dp[0][2] = ans[0] + b;
        dp[1][2] = ans[1] + b;

        ans[0] = dp[0][0].min(dp[1][0]);
        ans[1] = dp[0][1].min(dp[1][1]);
        ans[2] = dp[0][2].min(dp[1][2]);
    }

    print!("{}", ans.iter().min().unwrap());
}