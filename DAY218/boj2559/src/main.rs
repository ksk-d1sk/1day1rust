use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.trim_end().split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut prefix_sum = Vec::with_capacity(n);

    prefix_sum.push(next!(i32));

    for i in 0..(n - 1) {
        prefix_sum.push(next!(i32) + prefix_sum[i]);
    }

    let mut max = prefix_sum[k - 1];

    for i in k..n {
        max = max.max(prefix_sum[i] - prefix_sum[i - k]);
    }

    print!("{max}");
}