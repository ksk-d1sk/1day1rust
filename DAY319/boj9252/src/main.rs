use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let a = next!().as_bytes();
    let b = next!().as_bytes();
    let al = a.len();
    let bl = b.len();
    let mut dp = vec![vec![0; bl + 1]; al + 1];
    let mut stack = Vec::new();

    for i in 0..al {
        for j in 0..bl {
            if a[i] == b[j] {
                dp[i + 1][j + 1] = dp[i][j] + 1;
            } else {
                dp[i + 1][j + 1] = dp[i + 1][j].max(dp[i][j + 1]);
            }
        }
    }

    let mut i = al;
    let mut j = bl;

    while dp[i][j] != 0 {
        if dp[i][j] == dp[i - 1][j] {
            i -= 1;
        } else if dp[i][j] == dp[i][j - 1] {
            j -= 1;
        } else {
            i -= 1;
            j -= 1;
            stack.push(a[i]);
        }
    }

    while let Some(x) = stack.pop() {
        output.push(x as char);
    }

    print!("{}\n{output}", dp[al][bl]);
}