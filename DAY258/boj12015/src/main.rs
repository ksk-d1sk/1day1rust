use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let arr: Vec<_> = (0..n).map(|_| next!(i32)).collect();
    let mut dp = vec![1; n];
    let mut lis = vec![i32::MIN, arr[0]];

    for i in 1..n {
        if let Err(j) = lis.binary_search(&arr[i]) {
            if lis.get(j).is_some() {
                lis[j] = lis[j].min(arr[i]);
            } else {
                lis.push(arr[i]);
            }
            dp[i] = j;
        }
    }

    print!("{}", lis.len() - 1);
}