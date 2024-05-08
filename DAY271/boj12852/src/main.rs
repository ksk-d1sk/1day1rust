use std::io::*;
use std::fmt::Write;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $t:ty ) => { tokens.next().unwrap().parse::<$t>().unwrap() };
    }

    let n = next!(usize);
    let mut dp = vec![u8::MAX; n + 1];

    dp[1] = 0;

    for i in 2..=n {
        dp[i] = dp[i - 1] + 1;
        if i & 1 == 0 {
            dp[i] = dp[i].min(dp[i / 2] + 1);
        }
        if i % 3 == 0 {
            dp[i] = dp[i].min(dp[i / 3] + 1);
        }
    }

    let mut l = dp[n];
    let mut i = n;

    let _ = writeln!(output, "{l}");

    while i > 0 {
        let _ = write!(output, "{i} ");
        l -= 1;
        if dp[i - 1] == l {
            i -= 1;
        } else if i & 1 == 0 && dp[i / 2] == l {
            i /= 2;
        } else if dp[i / 3] == l {
            i /= 3;
        }
    }

    print!("{output}");
}