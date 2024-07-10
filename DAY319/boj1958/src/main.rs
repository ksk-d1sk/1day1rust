use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = next!().as_bytes();
    let b = next!().as_bytes();
    let c = next!().as_bytes();
    let al = a.len();
    let bl = b.len();
    let cl = c.len();
    let mut dp = vec![vec![vec![0; cl + 1]; bl + 1]; al + 1];

    for i in 1..=al {
        for j in 1..=bl {
            for k in 1..=cl {
                dp[i][j][k] = if a[i - 1] == b[j - 1] && b[j - 1] == c[k - 1] {
                    dp[i - 1][j - 1][k - 1] + 1
                } else {
                    dp[i - 1][j][k].max(dp[i][j - 1][k]).max(dp[i][j][k - 1])
                };
            }
        }
    }

    print!("{}", dp[al][bl][cl]);
}