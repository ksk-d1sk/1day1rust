use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = vec![1, 2];

    for i in 2..n {
        dp.push((dp[i - 1] + dp[i - 2]) % 15746);
    }

    println!("{}", dp[n - 1]);
}