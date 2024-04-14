use std::io::*;

const INF: u32 = u32::MAX;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr: Vec<_> = (0..n)
        .map(|_| next!(u32, u32, u32))
        .collect();
    let mut dp = vec![[INF; 9]; n];

    dp[0][0] = arr[0].0;
    dp[0][4] = arr[0].1;
    dp[0][8] = arr[0].2;

    for i in 1..n {
        dp[i][0] = min_add(dp[i - 1][3], dp[i - 1][6], arr[i].0);
        dp[i][1] = min_add(dp[i - 1][4], dp[i - 1][7], arr[i].0);
        dp[i][2] = min_add(dp[i - 1][5], dp[i - 1][8], arr[i].0);

        dp[i][3] = min_add(dp[i - 1][0], dp[i - 1][6], arr[i].1);
        dp[i][4] = min_add(dp[i - 1][1], dp[i - 1][7], arr[i].1);
        dp[i][5] = min_add(dp[i - 1][2], dp[i - 1][8], arr[i].1);

        dp[i][6] = min_add(dp[i - 1][0], dp[i - 1][3], arr[i].2);
        dp[i][7] = min_add(dp[i - 1][1], dp[i - 1][4], arr[i].2);
        dp[i][8] = min_add(dp[i - 1][2], dp[i - 1][5], arr[i].2);
    }

    dp[n - 1][0] = INF;
    dp[n - 1][4] = INF;
    dp[n - 1][8] = INF;

    print!("{}", dp[n - 1].iter().min().unwrap());
}

fn min_add(l: u32, r: u32, x: u32) -> u32 {
    if l == INF && r == INF {
        INF
    } else if l < r {
        l + x
    } else {
        r + x
    }
}