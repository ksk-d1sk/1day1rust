use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = vec![0_u64; n + 1];

    dp[0] = 1;

    for i in 1..=n {
        for j in 0..i {
            dp[i] += dp[j] * dp[i - j - 1];
        }
    }

    print!("{}", dp[n]);
}