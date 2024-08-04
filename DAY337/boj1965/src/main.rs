use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let v = Vec::from_iter((0..n).map(|_| next!(u16)));
    let mut dp = vec![1; n];
    
    for i in 1..n {
        for j in 0..i {
            if v[j] < v[i] {
                dp[i] = dp[i].max(dp[j] + 1);
            }
        }
    }

    println!("{}", dp.iter().max().unwrap());
}