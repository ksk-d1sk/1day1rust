use std::io::*;

fn main() {
    let input = read_to_string(stdin()).unwrap();
    let mut tokens = input.split_ascii_whitespace();

    macro_rules! next {
        () => { tokens.next().unwrap() };
        ( $($t:ty),+ ) => { ($(tokens.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let (n, k) = next!(usize, usize);
    let mut v = Vec::from_iter((0..n).map(|_| next!(u64)));
    let mut answer = 0;

    v.sort_unstable();

    answer += v[n - 1];

    for (i, j) in (0..(k-1)/2).map(|i| (i, n - i - 2)) {
        answer += v[j] - v[i];
    }

    print!("{answer}");
}