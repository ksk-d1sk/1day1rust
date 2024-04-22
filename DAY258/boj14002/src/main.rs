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

    let n = next!(usize);
    let arr: Vec<_> = (0..n).map(|_| next!(u16)).collect();
    let mut dp = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if arr[i] > arr[j] {
                dp[i] = dp[i].max(dp[j] + 1_u16);
            }
        }
    }

    let max = dp.iter().max().unwrap();
    let mut stack = Vec::new();
    let mut i = *max;

    for j in (0..n).rev() {
        if i == dp[j] {
            stack.push(arr[j]);
            i -= 1;
        }
    }

    while let Some(x) = stack.pop() {
        let _ = write!(output, "{x} ");
    }

    print!("{max}\n{output}");
}