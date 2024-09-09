use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let m = next!(u8);
    let mut dp = vec![1, 1];
    let mut t = 0;
    let mut answer = 1;

    for i in 2..=n {
        dp.push(dp[i - 1] + dp[i - 2]);
    }

    for x in (0..m).map(|_| next!(usize)) {
        answer *= dp[x - t - 1];
        t = x;
    }

    answer *= dp[n - t];

    print!("{answer}");
}