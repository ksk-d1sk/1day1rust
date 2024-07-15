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

    let (n, q) = next!(usize, usize);
    let mut v = Vec::from_iter((0..n).map(|_| next!(usize)));

    v.sort_unstable();

    let mut prefix_sum = vec![v[0]];

    for i in 1..n {
        prefix_sum.push(prefix_sum[i - 1] + v[i]);
    }

    for x in (0..q).map(|_| next!(usize)) {
        let _ = match v.binary_search(&x) {
            Ok(i) => {
                let left = (i + 1) * x - prefix_sum[i];
                let right = prefix_sum[n - 1] - prefix_sum[i] - (n - i - 1) * x;
                writeln!(output, "{}", left + right)
            }
            Err(i) => {
                let left = i * x - prefix_sum.get(i - 1).unwrap_or(&0);
                let right = prefix_sum[n - 1] - prefix_sum.get(i - 1).unwrap_or(&0) - (n - i) * x;
                writeln!(output, "{}", left + right)
            }
        };
    }

    print!("{output}");
}