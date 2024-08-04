use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (r, c, w) = next!(usize, usize, usize);
    let mut dp = vec![vec![1_u64]];
    let mut answer = 0;

    for i in 1..31 {
        let mut temp = vec![1; i + 1];
        for j in 1..i {
            temp[j] = dp[i - 1][j] + dp[i - 1][j - 1];
        }
        dp.push(temp);
    }

    for i in 0..w {
        for j in 0..=i {
            answer += dp[i + r - 1][j + c - 1];
        }
    }

    print!("{answer}");
}