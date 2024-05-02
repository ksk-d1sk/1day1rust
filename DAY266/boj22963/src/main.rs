use std::fmt::Write;
use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = next!(usize);
    let mut arr = Vec::from_iter((0..n).map(|_| next!(i32)));
    let mut dp = vec![0; n];
    let mut lis = vec![i32::MIN];

    for i in 0..n {
        let idx = lis.partition_point(|x| *x <= arr[i]);
        if lis.get(idx).is_some() {
            lis[idx] = lis[idx].min(arr[i]);
        } else {
            lis.push(arr[i]);
        }
        dp[i] = idx;
    }

    let mut l = lis.len() - 1;
    if n - l <= 3 {
        let _ = writeln!(output, "YES\n{}", n - l);
        for i in (0..n).rev() {
            if l == dp[i] {
                l -= 1;
            } else {
                arr[i] = arr.get(i + 1).copied().unwrap_or(1_000_000_000);
                let _ = writeln!(
                    output,
                    "{} {}",
                    i + 1,
                    arr[i]
                );
            }
        }
    } else {
        let _ = writeln!(output, "NO");
    }

    print!("{output}");
}