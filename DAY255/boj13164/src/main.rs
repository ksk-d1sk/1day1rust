use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let arr: Vec<_> = (0..n).map(|_| next!(u32)).collect();
    let mut diff_arr = vec![0; n - 1];

    for i in 1..n {
        diff_arr[i - 1] = arr[i] - arr[i - 1];
    }

    diff_arr.sort_unstable_by(|a, b| b.cmp(a));

    print!("{}", diff_arr.iter().skip(k - 1).sum::<u32>());
}