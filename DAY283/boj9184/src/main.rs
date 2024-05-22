use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let mut dp = vec![vec![vec![1048576; 101]; 101]; 101];

    for a in 0.. 101 {
        for b in 0..101 {
            for c in 0..101 {
                dp[a][b][c] = if a <= 50 || b <= 50 || c <= 50 {
                    1
                } else if a > 70 || b > 70 || c > 70 {
                    dp[70][70][70]
                } else if a < b && b < c {
                    dp[a][b][c-1] + dp[a][b-1][c-1] - dp[a][b-1][c]
                } else {
                    dp[a-1][b][c] + dp[a-1][b-1][c] + dp[a-1][b][c-1] - dp[a-1][b-1][c-1]
                };
            }
        }
    }

    loop {
        let (a, b, c) = next!(i32, i32, i32);
        if a == -1 && b == -1 && c == -1 {
            break;
        }

        let _ = writeln!(output, "w({a}, {b}, {c}) = {}", dp[a as usize + 50][b as usize + 50][c as usize + 50]);
    }

    print!("{output}");
}