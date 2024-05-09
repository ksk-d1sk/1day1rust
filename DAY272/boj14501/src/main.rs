use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr = Vec::from_iter((0..n).map(|_| next!(usize, u16)));
    let mut dp = vec![0; n];

    for i in (0..n).rev() {
        dp[i] = dp.get(i + 1).copied().unwrap_or(0).max(
            if i + arr[i].0 <= n {
                arr[i].1 + dp.get(i + arr[i].0).unwrap_or(&0)
            } else {
                0
            }
        );
    }

    print!("{}", dp[0]);
}