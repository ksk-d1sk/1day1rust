use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m, mut k) = next!(usize, usize, usize);

    if k == 0 {
        k = 1;
    }

    let mut dp = [[0_u32; 15]; 15];
    let (mr, mc) = ((k - 1) / m, (k - 1) % m);
    let (er, ec) = (n - 1, m - 1);

    for i in 0..15 {
        dp[0][i] = 1;
        dp[i][0] = 1;
    }

    for i in 1..15 {
        for j in 1..15 {
            dp[i][j] = dp[i - 1][j] + dp[i][j - 1];
        }
    }

    print!("{}", dp[mr][mc] * dp[er - mr][ec - mc]);
}