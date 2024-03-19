use std::cmp;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut dp = vec![vec![0; k + 1]; n + 1];

    for i in 1..=n {
        let (w, v) = next!(usize, u32);

        for j in 1..=k {
            dp[i][j] = if w > j {
                *dp.get(i - 1).and_then(|v| v.get(j)).unwrap_or(&0)
            } else {
                cmp::max(
                    *dp.get(i - 1).and_then(|v| v.get(j)).unwrap_or(&0),
                    v + *dp.get(i - 1).and_then(|v| v.get(j - w)).unwrap_or(&0)
                )
            };
        }
    }

    print!("{}", dp[n][k]);
}