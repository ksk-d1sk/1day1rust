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

    let t = next!(usize);
    for case in 1..=t {
        let (n, k) = next!(usize, usize);
        let arr = Vec::from_iter((0..n).map(|_| next!(i32)));
        let mut lis = vec![i32::MIN];

        for i in 0..n {
            if let Err(idx) = lis.binary_search(&arr[i]) {
                if lis.get(idx).is_some() {
                    lis[idx] = lis[idx].min(arr[i]);
                } else {
                    lis.push(arr[i]);
                }
            }
        }

        let _ = if lis.len() - 1 >= k {
            writeln!(output, "Case #{case}\n1")
        } else {
            writeln!(output, "Case #{case}\n0")
        };
    }

    print!("{output}");
}