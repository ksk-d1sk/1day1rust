use std::fmt::Write;
use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input  = buf.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let t = get!(usize);
    for x in 1..=t {
        let (p, q) = get!(usize, u32);

        let mut dp = Vec::with_capacity(p + 1);

        dp.push(0);
        dp.push(1);

        for i in 2..=p {
            dp.push((dp[i - 1] + dp[i - 2]) % q);
        }

        let _ = writeln!(output, "Case #{}: {}", x, dp[p] % q);
    }

    print!("{output}");
}