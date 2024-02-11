use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = vec![vec![0; n]; n];
    let mut original = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..=i {
            original[i][j] = next!(u32);
        }
    }

    dp[0][0] = original[0][0];

    for i in 0..(n - 1) {
        for j in 0..=i {
            dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + original[i + 1][j]);
            dp[i + 1][j + 1] = dp[i][j] + original[i + 1][j + 1];
        }
    }

    print!("{}", dp[n - 1].iter().max().unwrap());
}