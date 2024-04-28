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
    let arr = Vec::from_iter((0..n).map(|_| next!(u16)));
    let mut dp1 = vec![1; n];
    let mut dp2 = vec![1; n];

    for i in 1..n {
        for j in 0..i {
            if arr[i] > arr[j] {
                dp1[i] = dp1[i].max(dp1[j] + 1);
            }
        }
    }

    for i in 1..n {
        for j in 0..i {
            let i = n - i - 1;
            let j = n - j - 1;
            if arr[i] < arr[j] {
                dp2[i] = dp2[i].max(dp2[j] + 1);
            }
        }
    }

    for a in (0..q).map(|_| next!(usize) - 1) {
        let _ = writeln!(output, "{}", dp1[a] + dp2[a] - 1);
    }

    print!("{output}");
}