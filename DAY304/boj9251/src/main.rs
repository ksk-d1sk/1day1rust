use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
    }

    let a = next!().as_bytes();
    let b = next!().as_bytes();
    let al = a.len();
    let bl = b.len();
    let mut dp = vec![vec![0; bl + 1]; al + 1];

    for i in 1..=al {
        for j in 1..=bl {
            if a[i - 1] == b[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    print!("{}", dp[al][bl]);
}