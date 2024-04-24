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
    let arr: Vec<_> = (0..n).map(|_| next!(i32)).collect();
    let mut dp = vec![0; n];
    let mut lis = vec![i32::MIN];

    for i in 0..n {
        if let Err(j) = lis.binary_search(&arr[i]) {
            if lis.get(j).is_some() {
                lis[j] = lis[j].min(arr[i]);
            } else {
                lis.push(arr[i]);
            }
            dp[i] = j;
        }
    }

    let max = lis.len() - 1;
    let mut i = max;
    let mut stack = Vec::with_capacity(max);

    for j in (0..n).rev() {
        if i == 0 {
            break;
        } else if i == dp[j] {
            stack.push(arr[j]);
            i -= 1;
        }
    }

    while let Some(x) = stack.pop() {
        let _ = write!(output, "{x} ");
    }

    print!("{max}\n{output}");
}