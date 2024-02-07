use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = vec![vec![0_u64; n]; n];
    let board: Vec<Vec<_>> = (0..n)
        .map(|_| (0..n)
            .map(|_| next!(usize))
            .collect())
        .collect();

    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..n {
            if dp[i][j] != 0 && !(i == n - 1 && j == n - 1) {
                let next = board[i][j];

                if i + next < n {
                    dp[i + next][j] += dp[i][j];
                }

                if j + next < n {
                    dp[i][j + next] += dp[i][j];
                }
            }
        }
    }

    print!("{}", dp[n-1][n-1]);
}