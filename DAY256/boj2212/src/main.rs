use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut coordi_arr: Vec<_> = (0..n).map(|_| next!(i32)).collect();
    let mut diff_arr = vec![0; n - 1];

    coordi_arr.sort_unstable();

    for i in 1..n {
        diff_arr[i - 1] = coordi_arr[i] - coordi_arr[i - 1];
    }

    diff_arr.sort_unstable_by(|a, b| b.cmp(a));

    print!("{}", diff_arr.iter().skip(k - 1).sum::<i32>());
}