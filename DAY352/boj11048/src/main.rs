use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, m) = next!(usize, usize);
    let v: Vec<Vec<_>> = (0..n).map(|_| (0..m).map(|_| next!(u32)).collect()).collect();
    let mut dp = vec![vec![0; m]; n];

    dp[0][0] = v[0][0];

    for i in 0..n {
        for j in 0..m {
            for (ni, nj) in [(i + 1, j), (i, j + 1), (i + 1, j + 1)] {
                if ni < n && nj < m {
                    dp[ni][nj] = dp[ni][nj].max(dp[i][j] + v[ni][nj]);
                }
            }
        }
    }

    print!("{}", dp[n - 1][m - 1]);
}