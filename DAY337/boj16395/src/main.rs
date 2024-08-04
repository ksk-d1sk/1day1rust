use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut dp = vec![vec![1_u64]];

    for i in 1..n {
        let mut temp = vec![1; i + 1];
        for j in 1..i {
            temp[j] = dp[i - 1][j] + dp[i - 1][j - 1];
        }
        dp.push(temp);
    }

    print!("{}", dp[n - 1][k - 1]);
}