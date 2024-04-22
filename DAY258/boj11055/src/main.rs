use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr: Vec<_> = (0..n).map(|_| next!(u32)).collect();
    let mut dp = arr.clone();

    for i in 1..n {
        for j in 0..i {
            if arr[i] > arr[j] {
                dp[i] = dp[i].max(dp[j] + arr[i]);
            }
        }
    }

    print!("{}", dp.iter().max().unwrap());
}