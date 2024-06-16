use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = [0; 1001];

    dp[2] = 2;
    dp[3] = 5;

    for i in 4..=1000 {
        dp[i] = dp[i - 2] + if i & 1 == 1 {
            (i - 1) * 2
        } else {
            i * 2 - 2
        };
    }

    print!("{}", dp[n]);
}