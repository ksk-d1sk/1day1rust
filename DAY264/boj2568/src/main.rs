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
    let mut arr = Vec::from_iter((0..n).map(|_| next!(i32, i32)));
    let mut dp = vec![0; n];
    let mut lis = vec![i32::MIN];

    arr.sort_unstable_by_key(|k| k.0);

    for i in 0..n {
        match lis.binary_search(&arr[i].1) {
            Ok(idx) => dp[i] = idx,
            Err(idx) => {
                if lis.get(idx).is_some() {
                    lis[idx] = lis[idx].min(arr[i].1);
                } else {
                    lis.push(arr[i].1);
                }
                dp[i] = idx;
            }
        }
    }

    let mut l = lis.len() - 1;
    let mut v = Vec::new();

    let _ = writeln!(output, "{}", n - l);

    for i in (0..n).rev() {
        if dp[i] == l {
            l -= 1;
        } else {
            v.push(arr[i].0);
        }
    }

    v.sort_unstable();

    for x in v {
        let _ = writeln!(output, "{x}");
    }

    print!("{output}");
}