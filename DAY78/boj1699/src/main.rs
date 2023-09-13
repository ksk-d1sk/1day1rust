use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    print!("{}", solve(n));
}

fn solve(n: usize) -> u8 {
    let mut dp = vec![0; n + 1];

    dp[1] = 1;

    for i in 2..=n {
        let mut min_num = u8::MAX;

        for j in (1..).map(|x| x * x).take_while(|&x| x <= i) {
            if dp[i - j] < 4 {
                min_num = min_num.min(dp[i - j] + 1);
            }
        }

        dp[i] = min_num;
    }

    dp[n]
}