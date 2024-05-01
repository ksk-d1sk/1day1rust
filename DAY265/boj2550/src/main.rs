use std::collections::HashMap;
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
    let left = Vec::from_iter((0..n).map(|_| next!(u16)));
    let right: HashMap<_, _> = HashMap::from_iter((1..=n).map(|i| (next!(u16), i)));
    let arr: Vec<_> = left.iter().map(|i| right[i]).collect();
    let mut dp = vec![0; n];
    let mut lis = vec![0];

    for i in 0..n {
        match lis.binary_search(&arr[i]) {
            Ok(idx) => dp[i] = idx,
            Err(idx) => {
                if lis.get(idx).is_some() {
                    lis[idx] = lis[idx].min(arr[i]);
                } else {
                    lis.push(arr[i]);
                }
                dp[i] = idx;
            }
        }
    }

    let mut l = lis.len() - 1;
    let mut v = Vec::new();

    let _ = writeln!(output, "{l}");

    for i in (0..n).rev() {
        if l == 0 {
            break;
        } else if l == dp[i] {
            v.push(left[i]);
            l -= 1;
        }
    }

    v.sort_unstable();

    for x in v {
        let _ = write!(output, "{x} ");
    }

    print!("{output}");
}