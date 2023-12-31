use std::io;

fn main() {
    let buf = io::read_to_string(io::stdin()).unwrap();
    let mut input = buf.split_ascii_whitespace();

    macro_rules! get {
        ( $($t:ty),+ ) => { ($(input.next().unwrap().parse::<$t>().unwrap()),+) };
    }

    let n = get!(usize);
    let v: Vec<usize> = (0..n).map(|_| get!(usize)).collect();
    let diff_v: Vec<_> = v.windows(2).map(|slice| slice[1] - slice[0]).collect();
    let mut gcd = diff_v[0];

    for i in 1..diff_v.len() {
        gcd = euclidean(gcd, diff_v[i]);
    }

    print!("{}", (v[n - 1] - v[0]) / gcd + 1 - n);
}

fn euclidean(a: usize, b: usize) -> usize {
    let c = a % b;
    if c == 0 { b } else { euclidean(b, c) }
}