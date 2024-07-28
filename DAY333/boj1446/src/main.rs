use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, d) = next!(u8, usize);
    let shortcut = Vec::from_iter((0..n).map(|_| next!(usize, usize, u32)));
    let mut dp = vec![0; d + 1];

    for i in 1..=d {
        let mut min = dp[i - 1] + 1;

        for &(s, e, w) in shortcut.iter() {
            if i == e {
                min = min.min(dp[s] + w);
            }
        }

        dp[i] = min;
    }

    print!("{}", dp[d]);
}