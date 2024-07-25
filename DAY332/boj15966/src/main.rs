use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut dp = vec![0_u32; 1_000_001];
    let mut answer = 0;

    for x in (0..n).map(|_| next!(usize)) {
        dp[x] = dp[x - 1] + 1;
        answer = answer.max(dp[x]);
    }

    print!("{answer}");
}