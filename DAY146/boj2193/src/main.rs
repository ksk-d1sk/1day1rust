use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let mut dp = vec![1_u64; n + 1];

    for i in 3..=n {
        dp[i] = dp[i - 1] + dp[i - 2];
    }

    print!("{}", dp[n]);
}