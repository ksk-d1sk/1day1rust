use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n).map(|_| next!(u32)));
    let mut dp = vec![u32::MAX; n + 1];

    dp[0] = 0;

    for i in 1..=n {
        for j in 0..i {
            dp[i] = dp[i].min(dp[i - j - 1] + v[j]);
        }
    }

    print!("{}", dp[n]);
}