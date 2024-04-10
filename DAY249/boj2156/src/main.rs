use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr: Vec<_> = (0..n).map(|_| next!(u32)).collect();
    let mut dp = vec![[0; 3]; n];

    dp[0][1] = arr[0];

    for i in 1..n {
        dp[i][0] = *dp[i - 1].iter().max().unwrap();
        dp[i][1] = dp[i - 1][0] + arr[i];
        dp[i][2] = dp[i - 1][1] + arr[i];
    }

    print!("{}", dp[n - 1].iter().max().unwrap());
}