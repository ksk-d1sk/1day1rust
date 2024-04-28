use std::io::*;
use std::fmt::Write;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();
    let mut output = String::new();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, q) = next!(usize, usize);
    let arr = Vec::from_iter((0..n).map(|_| next!(i32)));
    let mut dp1 = vec![0; n];
    let mut dp2 = vec![0; n];
    let mut lis1 = vec![i32::MIN];
    let mut lis2 = vec![i32::MAX];

    for i in 0..n {
        match lis1.binary_search(&arr[i]) {
            Ok(idx) => dp1[i] = idx,
            Err(idx) => {
                if lis1.get(idx).is_some() {
                    lis1[idx] = lis1[idx].min(arr[i]);
                } else {
                    lis1.push(arr[i]);
                }
                dp1[i] = idx;
            }
        }
    }

    for i in 0..n {
        let i = n - i - 1;
        match lis2.binary_search_by(|probe| arr[i].cmp(probe)) {
            Ok(idx) => dp2[i] = idx,
            Err(idx) => {
                if lis2.get(idx).is_some() {
                    lis2[idx] = lis2[idx].max(arr[i]);
                } else {
                    lis2.push(arr[i]);
                }
                dp2[i] = idx;
            }
        }
    }

    for a in (0..q).map(|_| next!(usize) - 1) {
        let _ = writeln!(output, "{}", dp1[a] + dp2[a] - 1);
    }

    print!("{output}");
}